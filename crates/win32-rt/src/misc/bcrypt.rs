//! bcrypt.dll (CNG): random numbers, hashing (MD2, MD4, MD5, SHA-1/256/384/512,
//! HMAC, reusable hashes, AES-CMAC, AES-GMAC), PBKDF2, symmetric ciphers
//! (AES-128/192/256 ECB/CBC/CFB/CTR/GCM/CCM + RFC 3394 Key Wrap, 3DES, 3DES_112,
//! DES, RC2, RC4), RSA (PKCS#1 v1.5 / PSS sign & verify, PKCS#1 v1.5 / OAEP /
//! raw encrypt & decrypt, CNG & CAPI key blobs), ECDSA P-256 / P-384 / P-521,
//! and ECDH P-256 / P-384 / P-521 (`BCryptSecretAgreement` + `BCryptDeriveKey`).
//!
//! Digests and AES/DES/3DES primitives use CommonCrypto (libSystem); asymmetric
//! keygen, ECDSA sign/verify, ECDH key exchange, and RSA private-key operations
//! use Apple's `Security.framework` (`SecKey`), which is available identically
//! on macOS and iOS. RSA public-key operations and small-key fallbacks also
//! retain a pure-Rust big-integer `modexp` path supporting arbitrary PSS salt
//! lengths, OAEP labels, and 512/1024-bit keys.

use std::ffi::c_void;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

const STATUS_SUCCESS: i32 = 0;
const STATUS_INVALID_PARAMETER: i32 = 0xC000_000Du32 as i32;
const STATUS_INVALID_HANDLE: i32 = 0xC000_0008u32 as i32;
const STATUS_NOT_SUPPORTED: i32 = 0xC000_00BBu32 as i32;
const STATUS_NOT_FOUND: i32 = 0xC000_0225u32 as i32;
const STATUS_BUFFER_TOO_SMALL: i32 = 0xC000_0023u32 as i32;
const STATUS_INVALID_BUFFER_SIZE: i32 = 0xC000_0206u32 as i32;
const STATUS_DATA_ERROR: i32 = 0xC000_003Eu32 as i32;
const STATUS_INVALID_SIGNATURE: i32 = 0xC000_A000u32 as i32;
const STATUS_AUTH_TAG_MISMATCH: i32 = 0xC000_A002u32 as i32;

const BCRYPT_ALG_HANDLE_HMAC_FLAG: u32 = 0x8;
const BCRYPT_HASH_REUSABLE_FLAG: u32 = 0x20;
const BCRYPT_BLOCK_PADDING: u32 = 0x1;
const BCRYPT_PAD_NONE: u32 = 0x1;
const BCRYPT_PAD_PKCS1: u32 = 0x2;
const BCRYPT_PAD_OAEP: u32 = 0x4;
const BCRYPT_PAD_PSS: u32 = 0x8;
const BCRYPT_PAD_PKCS1_OPTIONAL_HASH_OID: u32 = 0x40;
const BCRYPT_SUPPORTED_PAD_PKCS1_ENC: u32 = 0x0000_0002;
const BCRYPT_SUPPORTED_PAD_PKCS1_SIG: u32 = 0x0000_0004;
const BCRYPT_SUPPORTED_PAD_OAEP: u32 = 0x0000_0008;
const BCRYPT_SUPPORTED_PAD_PSS: u32 = 0x0000_0010;

const BCRYPT_CIPHER_INTERFACE: u32 = 1;
const BCRYPT_HASH_INTERFACE: u32 = 2;
const BCRYPT_ASYMMETRIC_ENCRYPTION_INTERFACE: u32 = 3;
const BCRYPT_SECRET_AGREEMENT_INTERFACE: u32 = 4;
const BCRYPT_SIGNATURE_INTERFACE: u32 = 5;
const BCRYPT_RNG_INTERFACE: u32 = 6;
const BCRYPT_KEY_DERIVATION_INTERFACE: u32 = 7;

const BCRYPT_RSAPUBLIC_MAGIC: u32 = 0x3141_5352; // "RSA1"
const BCRYPT_RSAPRIVATE_MAGIC: u32 = 0x3241_5352; // "RSA2"
const BCRYPT_RSAFULLPRIVATE_MAGIC: u32 = 0x3341_5352; // "RSA3"
const BCRYPT_ECDSA_PUBLIC_P256_MAGIC: u32 = 0x3153_4345; // "ECS1"
const BCRYPT_ECDSA_PRIVATE_P256_MAGIC: u32 = 0x3253_4345; // "ECS2"
const BCRYPT_ECDSA_PUBLIC_P384_MAGIC: u32 = 0x3353_4345; // "ECS3"
const BCRYPT_ECDSA_PRIVATE_P384_MAGIC: u32 = 0x3453_4345; // "ECS4"
const BCRYPT_ECDSA_PUBLIC_P521_MAGIC: u32 = 0x3553_4345; // "ECS5"
const BCRYPT_ECDSA_PRIVATE_P521_MAGIC: u32 = 0x3653_4345; // "ECS6"
const BCRYPT_ECDSA_PUBLIC_GENERIC_MAGIC: u32 = 0x5044_4345; // "ECDP"
const BCRYPT_ECDSA_PRIVATE_GENERIC_MAGIC: u32 = 0x5644_4345; // "ECDV"
const BCRYPT_ECDH_PUBLIC_P256_MAGIC: u32 = 0x314B_4345; // "ECK1"
const BCRYPT_ECDH_PRIVATE_P256_MAGIC: u32 = 0x324B_4345; // "ECK2"
const BCRYPT_ECDH_PUBLIC_P384_MAGIC: u32 = 0x334B_4345; // "ECK3"
const BCRYPT_ECDH_PRIVATE_P384_MAGIC: u32 = 0x344B_4345; // "ECK4"
const BCRYPT_ECDH_PUBLIC_P521_MAGIC: u32 = 0x354B_4345; // "ECK5"
const BCRYPT_ECDH_PRIVATE_P521_MAGIC: u32 = 0x364B_4345; // "ECK6"
const BCRYPT_ECDH_PUBLIC_GENERIC_MAGIC: u32 = 0x504B_4345; // "ECKP"
const BCRYPT_ECDH_PRIVATE_GENERIC_MAGIC: u32 = 0x564B_4345; // "ECKV"
const BCRYPT_KEY_DATA_BLOB_MAGIC: u32 = 0x4D42_444B; // "KDBM"
const BCRYPT_OPAQUE_KEY_MAGIC: u32 = 0x5150_4F42; // "BOPQ"
/// Size we report for BCRYPT_OBJECT_LENGTH; the caller's buffer is not used
/// (our objects live on the Rust heap).
const OBJECT_LENGTH: u32 = 64;

// ---------------------------------------------------------------------------
// Digests (CommonCrypto)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Digest {
    Md2,
    Md4,
    Md5,
    Sha1,
    Sha256,
    Sha384,
    Sha512,
}

extern "C" {
    fn CC_MD2(data: *const c_void, len: u32, md: *mut u8) -> *mut u8;
    fn CC_MD4(data: *const c_void, len: u32, md: *mut u8) -> *mut u8;
    fn CC_MD5(data: *const c_void, len: u32, md: *mut u8) -> *mut u8;
    fn CC_SHA1(data: *const c_void, len: u32, md: *mut u8) -> *mut u8;
    fn CC_SHA256(data: *const c_void, len: u32, md: *mut u8) -> *mut u8;
    fn CC_SHA384(data: *const c_void, len: u32, md: *mut u8) -> *mut u8;
    fn CC_SHA512(data: *const c_void, len: u32, md: *mut u8) -> *mut u8;
    fn CC_MD2_Init(c: *mut c_void) -> i32;
    fn CC_MD2_Update(c: *mut c_void, data: *const c_void, len: u32) -> i32;
    fn CC_MD2_Final(md: *mut u8, c: *mut c_void) -> i32;
    fn CC_MD4_Init(c: *mut c_void) -> i32;
    fn CC_MD4_Update(c: *mut c_void, data: *const c_void, len: u32) -> i32;
    fn CC_MD4_Final(md: *mut u8, c: *mut c_void) -> i32;
    fn CC_MD5_Init(c: *mut c_void) -> i32;
    fn CC_MD5_Update(c: *mut c_void, data: *const c_void, len: u32) -> i32;
    fn CC_MD5_Final(md: *mut u8, c: *mut c_void) -> i32;
    fn CC_SHA1_Init(c: *mut c_void) -> i32;
    fn CC_SHA1_Update(c: *mut c_void, data: *const c_void, len: u32) -> i32;
    fn CC_SHA1_Final(md: *mut u8, c: *mut c_void) -> i32;
    fn CC_SHA256_Init(c: *mut c_void) -> i32;
    fn CC_SHA256_Update(c: *mut c_void, data: *const c_void, len: u32) -> i32;
    fn CC_SHA256_Final(md: *mut u8, c: *mut c_void) -> i32;
    fn CC_SHA384_Init(c: *mut c_void) -> i32;
    fn CC_SHA384_Update(c: *mut c_void, data: *const c_void, len: u32) -> i32;
    fn CC_SHA384_Final(md: *mut u8, c: *mut c_void) -> i32;
    fn CC_SHA512_Init(c: *mut c_void) -> i32;
    fn CC_SHA512_Update(c: *mut c_void, data: *const c_void, len: u32) -> i32;
    fn CC_SHA512_Final(md: *mut u8, c: *mut c_void) -> i32;
    fn CCHmac(alg: u32, key: *const c_void, key_len: usize, data: *const c_void, data_len: usize, mac: *mut u8);
    fn CCCrypt(
        op: u32,
        alg: u32,
        options: u32,
        key: *const c_void,
        key_length: usize,
        iv: *const c_void,
        data_in: *const c_void,
        data_in_length: usize,
        data_out: *mut c_void,
        data_out_available: usize,
        data_out_moved: *mut usize,
    ) -> i32;
}

impl Digest {
    pub(crate) fn from_name(n: &str) -> Option<Digest> {
        Some(match n.to_ascii_uppercase().as_str() {
            "MD2" => Digest::Md2,
            "MD4" => Digest::Md4,
            "MD5" => Digest::Md5,
            "SHA1" => Digest::Sha1,
            "SHA256" => Digest::Sha256,
            "SHA384" => Digest::Sha384,
            "SHA512" => Digest::Sha512,
            _ => return None,
        })
    }
    fn as_name(self) -> &'static str {
        match self {
            Digest::Md2 => "MD2",
            Digest::Md4 => "MD4",
            Digest::Md5 => "MD5",
            Digest::Sha1 => "SHA1",
            Digest::Sha256 => "SHA256",
            Digest::Sha384 => "SHA384",
            Digest::Sha512 => "SHA512",
        }
    }
    pub fn len(self) -> usize {
        match self {
            Digest::Md2 | Digest::Md4 | Digest::Md5 => 16,
            Digest::Sha1 => 20,
            Digest::Sha256 => 32,
            Digest::Sha384 => 48,
            Digest::Sha512 => 64,
        }
    }
    fn block_len(self) -> u32 {
        match self {
            Digest::Md2 => 16,
            Digest::Sha384 | Digest::Sha512 => 128,
            _ => 64,
        }
    }
    pub fn hash(self, data: &[u8]) -> Vec<u8> {
        let mut out = vec![0u8; self.len()];
        let f = match self {
            Digest::Md2 => CC_MD2,
            Digest::Md4 => CC_MD4,
            Digest::Md5 => CC_MD5,
            Digest::Sha1 => CC_SHA1,
            Digest::Sha256 => CC_SHA256,
            Digest::Sha384 => CC_SHA384,
            Digest::Sha512 => CC_SHA512,
        };
        assert!(data.len() <= u32::MAX as usize);
        unsafe { f(data.as_ptr() as *const c_void, data.len() as u32, out.as_mut_ptr()) };
        out
    }
    pub fn hash_slices(self, slices: &[&[u8]]) -> Vec<u8> {
        let mut ctx = [0u64; 64];
        let ctx_p = ctx.as_mut_ptr() as *mut c_void;
        let mut out = vec![0u8; self.len()];
        let (init, update, fin): (
            unsafe extern "C" fn(*mut c_void) -> i32,
            unsafe extern "C" fn(*mut c_void, *const c_void, u32) -> i32,
            unsafe extern "C" fn(*mut u8, *mut c_void) -> i32,
        ) = match self {
            Digest::Md2 => (CC_MD2_Init, CC_MD2_Update, CC_MD2_Final),
            Digest::Md4 => (CC_MD4_Init, CC_MD4_Update, CC_MD4_Final),
            Digest::Md5 => (CC_MD5_Init, CC_MD5_Update, CC_MD5_Final),
            Digest::Sha1 => (CC_SHA1_Init, CC_SHA1_Update, CC_SHA1_Final),
            Digest::Sha256 => (CC_SHA256_Init, CC_SHA256_Update, CC_SHA256_Final),
            Digest::Sha384 => (CC_SHA384_Init, CC_SHA384_Update, CC_SHA384_Final),
            Digest::Sha512 => (CC_SHA512_Init, CC_SHA512_Update, CC_SHA512_Final),
        };
        unsafe {
            init(ctx_p);
            for s in slices {
                for chunk in s.chunks(16 * 1024 * 1024) {
                    update(ctx_p, chunk.as_ptr() as *const c_void, chunk.len() as u32);
                }
            }
            fin(out.as_mut_ptr(), ctx_p);
        }
        out
    }
    pub fn hmac(self, key: &[u8], data: &[u8]) -> Vec<u8> {
        let cc_alg = match self {
            Digest::Sha1 => Some(0),
            Digest::Md5 => Some(1),
            Digest::Sha256 => Some(2),
            Digest::Sha384 => Some(3),
            Digest::Sha512 => Some(4),
            Digest::Md2 | Digest::Md4 => None,
        };
        if let Some(alg) = cc_alg {
            let mut out = vec![0u8; self.len()];
            unsafe {
                CCHmac(alg, key.as_ptr() as *const c_void, key.len(), data.as_ptr() as *const c_void, data.len(), out.as_mut_ptr())
            };
            return out;
        }
        // RFC 2104 HMAC for MD2 / MD4 (block size 64 for MD4, 16 for MD2).
        let b = self.block_len() as usize;
        let mut k_pad = vec![0u8; b];
        if key.len() > b {
            let h = self.hash(key);
            k_pad[..h.len()].copy_from_slice(&h);
        } else {
            k_pad[..key.len()].copy_from_slice(key);
        }
        let ipad: Vec<u8> = k_pad.iter().map(|x| x ^ 0x36).collect();
        let opad: Vec<u8> = k_pad.iter().map(|x| x ^ 0x5c).collect();
        let inner = self.hash_slices(&[&ipad, data]);
        self.hash_slices(&[&opad, &inner])
    }
    /// DER DigestInfo prefix for EMSA-PKCS1-v1_5.
    fn digest_info_prefix(self) -> &'static [u8] {
        match self {
            Digest::Md2 => &[0x30, 0x20, 0x30, 0x0c, 0x06, 0x08, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x02, 0x02, 0x05, 0x00, 0x04, 0x10],
            Digest::Md4 => &[0x30, 0x20, 0x30, 0x0c, 0x06, 0x08, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x02, 0x04, 0x05, 0x00, 0x04, 0x10],
            Digest::Md5 => &[0x30, 0x20, 0x30, 0x0c, 0x06, 0x08, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x02, 0x05, 0x05, 0x00, 0x04, 0x10],
            Digest::Sha1 => &[0x30, 0x21, 0x30, 0x09, 0x06, 0x05, 0x2b, 0x0e, 0x03, 0x02, 0x1a, 0x05, 0x00, 0x04, 0x14],
            Digest::Sha256 => &[
                0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01, 0x05, 0x00, 0x04, 0x20,
            ],
            Digest::Sha384 => &[
                0x30, 0x41, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x02, 0x05, 0x00, 0x04, 0x30,
            ],
            Digest::Sha512 => &[
                0x30, 0x51, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x03, 0x05, 0x00, 0x04, 0x40,
            ],
        }
    }
}

/// Constant-space incremental CommonCrypto digest context (512 bytes) for streaming callers.
#[derive(Clone)]
pub(crate) struct IncrementalDigest {
    digest: Digest,
    ctx: [u64; 64],
}

impl IncrementalDigest {
    fn ops(
        d: Digest,
    ) -> (
        unsafe extern "C" fn(*mut c_void) -> i32,
        unsafe extern "C" fn(*mut c_void, *const c_void, u32) -> i32,
        unsafe extern "C" fn(*mut u8, *mut c_void) -> i32,
    ) {
        match d {
            Digest::Md2 => (CC_MD2_Init, CC_MD2_Update, CC_MD2_Final),
            Digest::Md4 => (CC_MD4_Init, CC_MD4_Update, CC_MD4_Final),
            Digest::Md5 => (CC_MD5_Init, CC_MD5_Update, CC_MD5_Final),
            Digest::Sha1 => (CC_SHA1_Init, CC_SHA1_Update, CC_SHA1_Final),
            Digest::Sha256 => (CC_SHA256_Init, CC_SHA256_Update, CC_SHA256_Final),
            Digest::Sha384 => (CC_SHA384_Init, CC_SHA384_Update, CC_SHA384_Final),
            Digest::Sha512 => (CC_SHA512_Init, CC_SHA512_Update, CC_SHA512_Final),
        }
    }

    pub(crate) fn new(digest: Digest) -> Self {
        let (init, _, _) = Self::ops(digest);
        let mut ctx = [0u64; 64];
        unsafe {
            init(ctx.as_mut_ptr() as *mut c_void);
        }
        Self { digest, ctx }
    }

    pub(crate) fn update(&mut self, data: &[u8]) {
        let (_, update, _) = Self::ops(self.digest);
        let ctx_p = self.ctx.as_mut_ptr() as *mut c_void;
        for chunk in data.chunks(16 * 1024 * 1024) {
            unsafe {
                update(ctx_p, chunk.as_ptr() as *const c_void, chunk.len() as u32);
            }
        }
    }

    pub(crate) fn finalize_clone(&self) -> Vec<u8> {
        let (_, _, fin) = Self::ops(self.digest);
        let mut tmp = self.ctx;
        let mut out = vec![0u8; self.digest.len()];
        unsafe {
            fin(out.as_mut_ptr(), tmp.as_mut_ptr() as *mut c_void);
        }
        out
    }
}

// ---------------------------------------------------------------------------
// Big integers (little-endian u32 limbs) for RSA public operations
// ---------------------------------------------------------------------------

fn bn_from_be(b: &[u8]) -> Vec<u32> {
    let mut v: Vec<u32> = b.rchunks(4).map(|c| c.iter().fold(0u32, |a, &x| (a << 8) | x as u32)).collect();
    while v.last() == Some(&0) {
        v.pop();
    }
    v
}

fn bn_to_be(v: &[u32], len: usize) -> Vec<u8> {
    let mut out = vec![0u8; len];
    for (i, limb) in v.iter().enumerate() {
        for k in 0..4 {
            let pos = i * 4 + k;
            if pos < len {
                out[len - 1 - pos] = (limb >> (8 * k)) as u8;
            }
        }
    }
    out
}

fn bn_bits(v: &[u32]) -> usize {
    v.iter().rposition(|&x| x != 0).map_or(0, |i| i * 32 + 32 - v[i].leading_zeros() as usize)
}

fn bn_bit(v: &[u32], i: usize) -> bool {
    v.get(i / 32).is_some_and(|x| x >> (i % 32) & 1 != 0)
}

fn bn_cmp(a: &[u32], b: &[u32]) -> std::cmp::Ordering {
    let n = a.len().max(b.len());
    for i in (0..n).rev() {
        let (x, y) = (a.get(i).copied().unwrap_or(0), b.get(i).copied().unwrap_or(0));
        if x != y {
            return x.cmp(&y);
        }
    }
    std::cmp::Ordering::Equal
}

/// a -= b (requires a >= b).
fn bn_sub_assign(a: &mut [u32], b: &[u32]) {
    let mut borrow = 0i64;
    for i in 0..a.len() {
        let d = a[i] as i64 - b.get(i).copied().unwrap_or(0) as i64 - borrow;
        if d < 0 {
            a[i] = (d + (1i64 << 32)) as u32;
            borrow = 1;
        } else {
            a[i] = d as u32;
            borrow = 0;
        }
    }
}

fn bn_mul(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut r = vec![0u32; a.len() + b.len()];
    for (i, &x) in a.iter().enumerate() {
        let mut carry = 0u64;
        for (j, &y) in b.iter().enumerate() {
            let t = r[i + j] as u64 + x as u64 * y as u64 + carry;
            r[i + j] = t as u32;
            carry = t >> 32;
        }
        r[i + b.len()] = carry as u32;
    }
    r
}

/// a mod n by binary long division (n != 0).
fn bn_mod(a: &[u32], n: &[u32]) -> Vec<u32> {
    let mut r = vec![0u32; n.len() + 1];
    for i in (0..bn_bits(a)).rev() {
        // r = (r << 1) | bit
        let mut carry = bn_bit(a, i) as u32;
        for limb in r.iter_mut() {
            let next = *limb >> 31;
            *limb = (*limb << 1) | carry;
            carry = next;
        }
        if bn_cmp(&r, n) != std::cmp::Ordering::Less {
            bn_sub_assign(&mut r, n);
        }
    }
    r.truncate(n.len());
    r
}

/// base^e mod n.
pub fn bn_modexp(base: &[u32], e: &[u32], n: &[u32]) -> Vec<u32> {
    let b = bn_mod(base, n);
    let mut r = vec![1u32];
    for i in (0..bn_bits(e)).rev() {
        r = bn_mod(&bn_mul(&r, &r), n);
        if bn_bit(e, i) {
            r = bn_mod(&bn_mul(&r, &b), n);
        }
    }
    r
}

fn bn_trim(v: &mut Vec<u32>) {
    while v.last() == Some(&0) {
        v.pop();
    }
}

fn bn_add(a: &[u32], b: &[u32]) -> Vec<u32> {
    let n = a.len().max(b.len());
    let mut r = Vec::with_capacity(n + 1);
    let mut carry = 0u64;
    for i in 0..n {
        let s = a.get(i).copied().unwrap_or(0) as u64 + b.get(i).copied().unwrap_or(0) as u64 + carry;
        r.push(s as u32);
        carry = s >> 32;
    }
    if carry != 0 {
        r.push(carry as u32);
    }
    bn_trim(&mut r);
    r
}

fn bn_sub(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut r = a.to_vec();
    bn_sub_assign(&mut r, b);
    bn_trim(&mut r);
    r
}

fn bn_divmod(a: &[u32], b: &[u32]) -> (Vec<u32>, Vec<u32>) {
    if b.is_empty() {
        return (Vec::new(), Vec::new());
    }
    let mut q = vec![0u32; a.len()];
    let mut r = vec![0u32; b.len() + 1];
    for i in (0..bn_bits(a)).rev() {
        let mut carry = bn_bit(a, i) as u32;
        for limb in r.iter_mut() {
            let next = *limb >> 31;
            *limb = (*limb << 1) | carry;
            carry = next;
        }
        if bn_cmp(&r, b) != std::cmp::Ordering::Less {
            bn_sub_assign(&mut r, b);
            q[i / 32] |= 1u32 << (i % 32);
        }
    }
    bn_trim(&mut q);
    bn_trim(&mut r);
    (q, r)
}

/// Modular inverse `a^-1 mod m` (returns None if `gcd(a, m) != 1`).
fn bn_modinv(a: &[u32], m: &[u32]) -> Option<Vec<u32>> {
    if m.is_empty() {
        return None;
    }
    let mut r0 = m.to_vec();
    let mut r1 = bn_mod(a, m);
    bn_trim(&mut r0);
    bn_trim(&mut r1);
    let mut t0: (Vec<u32>, bool) = (Vec::new(), false);
    let mut t1: (Vec<u32>, bool) = (vec![1], false);
    while !r1.is_empty() {
        let (q, rem) = bn_divmod(&r0, &r1);
        let mut prod = bn_mul(&q, &t1.0);
        bn_trim(&mut prod);
        let rhs_neg = !t1.1 && !prod.is_empty();
        let new_t = if t0.0.is_empty() {
            (prod, rhs_neg)
        } else if prod.is_empty() {
            t0.clone()
        } else if t0.1 == rhs_neg {
            (bn_add(&t0.0, &prod), t0.1)
        } else {
            match bn_cmp(&t0.0, &prod) {
                std::cmp::Ordering::Equal => (Vec::new(), false),
                std::cmp::Ordering::Greater => (bn_sub(&t0.0, &prod), t0.1),
                std::cmp::Ordering::Less => (bn_sub(&prod, &t0.0), rhs_neg),
            }
        };
        r0 = r1;
        r1 = rem;
        t0 = t1;
        t1 = new_t;
    }
    if r0 != [1] {
        return None;
    }
    let mut mag = bn_mod(&t0.0, m);
    bn_trim(&mut mag);
    if t0.1 && !mag.is_empty() {
        Some(bn_sub(m, &mag))
    } else {
        Some(mag)
    }
}

fn fill_random(buf: &mut [u8]) {
    if buf.is_empty() {
        return;
    }
    unsafe {
        if !crate::det_entropy::fill(buf.as_mut_ptr(), buf.len()) {
            libc::arc4random_buf(buf.as_mut_ptr() as *mut c_void, buf.len());
        }
    }
}

fn strip_leading_zeros(mut b: &[u8]) -> &[u8] {
    while b.len() > 1 && b[0] == 0 {
        b = &b[1..];
    }
    b
}

fn pad_be(b: &[u8], len: usize) -> Option<Vec<u8>> {
    let trimmed = strip_leading_zeros(b);
    if trimmed.len() > len {
        return None;
    }
    let mut out = vec![0u8; len];
    out[len - trimmed.len()..].copy_from_slice(trimmed);
    Some(out)
}

// ---------------------------------------------------------------------------
// Minimal ASN.1 DER helpers (for PKCS#1 RSA keys and ECDSA (r, s) signatures)
// ---------------------------------------------------------------------------

fn der_tlv(tag: u8, val: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(2 + 4 + val.len());
    out.push(tag);
    let len = val.len();
    if len < 0x80 {
        out.push(len as u8);
    } else if len <= 0xFF {
        out.push(0x81);
        out.push(len as u8);
    } else if len <= 0xFFFF {
        out.push(0x82);
        out.push((len >> 8) as u8);
        out.push(len as u8);
    } else {
        out.push(0x83);
        out.push((len >> 16) as u8);
        out.push((len >> 8) as u8);
        out.push(len as u8);
    }
    out.extend_from_slice(val);
    out
}

fn der_uint(be: &[u8]) -> Vec<u8> {
    let s = strip_leading_zeros(be);
    if s.is_empty() {
        return der_tlv(0x02, &[0]);
    }
    if (s[0] & 0x80) != 0 {
        let mut v = Vec::with_capacity(s.len() + 1);
        v.push(0);
        v.extend_from_slice(s);
        der_tlv(0x02, &v)
    } else {
        der_tlv(0x02, s)
    }
}

fn parse_der_tlv(buf: &[u8]) -> Option<(u8, &[u8], &[u8])> {
    if buf.len() < 2 {
        return None;
    }
    let tag = buf[0];
    let lb = buf[1];
    let (hdr, len) = if (lb & 0x80) == 0 {
        (2usize, lb as usize)
    } else {
        let n = (lb & 0x7F) as usize;
        if n == 0 || n > 4 || buf.len() < 2 + n {
            return None;
        }
        let mut v = 0usize;
        for &b in &buf[2..2 + n] {
            v = (v << 8) | (b as usize);
        }
        (2 + n, v)
    };
    let end = hdr.checked_add(len)?;
    if end > buf.len() {
        return None;
    }
    Some((tag, &buf[hdr..end], &buf[end..]))
}

fn parse_der_seq_uints(der: &[u8]) -> Option<Vec<Vec<u8>>> {
    let (tag, body, rest) = parse_der_tlv(der)?;
    if tag != 0x30 || !rest.is_empty() {
        return None;
    }
    let mut rem = body;
    let mut out = Vec::new();
    while !rem.is_empty() {
        let (t, val, next) = parse_der_tlv(rem)?;
        if t != 0x02 || val.is_empty() {
            return None;
        }
        out.push(strip_leading_zeros(val).to_vec());
        rem = next;
    }
    Some(out)
}

// ---------------------------------------------------------------------------
// Apple Security.framework (SecKey) backend — identical API on macOS & iOS
// ---------------------------------------------------------------------------

mod seckey {
    use super::*;

    const K_CF_NUMBER_SINT32_TYPE: isize = 3;

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        static kCFAllocatorDefault: *const c_void;
        static kCFBooleanFalse: *const c_void;
        static kCFTypeDictionaryKeyCallBacks: c_void;
        static kCFTypeDictionaryValueCallBacks: c_void;

        fn CFRelease(cf: *const c_void);
        fn CFDataCreate(allocator: *const c_void, bytes: *const u8, length: isize) -> *const c_void;
        fn CFDataGetLength(the_data: *const c_void) -> isize;
        fn CFDataGetBytePtr(the_data: *const c_void) -> *const u8;
        fn CFNumberCreate(allocator: *const c_void, the_type: isize, value_ptr: *const c_void) -> *const c_void;
        fn CFDictionaryCreate(
            allocator: *const c_void,
            keys: *const *const c_void,
            values: *const *const c_void,
            num_values: isize,
            key_callbacks: *const u8,
            value_callbacks: *const u8,
        ) -> *const c_void;
    }

    #[link(name = "Security", kind = "framework")]
    extern "C" {
        static kSecAttrKeyType: *const c_void;
        static kSecAttrKeyTypeRSA: *const c_void;
        static kSecAttrKeyTypeECSECPrimeRandom: *const c_void;
        static kSecAttrKeyClass: *const c_void;
        static kSecAttrKeyClassPublic: *const c_void;
        static kSecAttrKeyClassPrivate: *const c_void;
        static kSecAttrKeySizeInBits: *const c_void;
        static kSecAttrIsPermanent: *const c_void;

        static kSecKeyAlgorithmECDSASignatureDigestX962: *const c_void;
        static kSecKeyAlgorithmECDHKeyExchangeStandard: *const c_void;
        static kSecKeyAlgorithmRSASignatureRaw: *const c_void;

        fn SecKeyCreateRandomKey(parameters: *const c_void, error: *mut *const c_void) -> *const c_void;
        fn SecKeyCreateWithData(
            key_data: *const c_void,
            attributes: *const c_void,
            error: *mut *const c_void,
        ) -> *const c_void;
        fn SecKeyCopyExternalRepresentation(key: *const c_void, error: *mut *const c_void) -> *const c_void;
        fn SecKeyCreateSignature(
            key: *const c_void,
            algorithm: *const c_void,
            data_to_sign: *const c_void,
            error: *mut *const c_void,
        ) -> *const c_void;
        fn SecKeyVerifySignature(
            key: *const c_void,
            algorithm: *const c_void,
            signed_data: *const c_void,
            signature: *const c_void,
            error: *mut *const c_void,
        ) -> u8;
        fn SecKeyCopyKeyExchangeResult(
            private_key: *const c_void,
            algorithm: *const c_void,
            public_key: *const c_void,
            parameters: *const c_void,
            error: *mut *const c_void,
        ) -> *const c_void;
    }

    struct CfGuard(*const c_void);
    impl Drop for CfGuard {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe { CFRelease(self.0) };
            }
        }
    }

    unsafe fn make_cf_data(bytes: &[u8]) -> Option<CfGuard> {
        let p = CFDataCreate(kCFAllocatorDefault, bytes.as_ptr(), bytes.len() as isize);
        (!p.is_null()).then_some(CfGuard(p))
    }

    unsafe fn cf_data_to_vec(data: *const c_void) -> Vec<u8> {
        let len = CFDataGetLength(data);
        if len <= 0 {
            return Vec::new();
        }
        let ptr = CFDataGetBytePtr(data);
        if ptr.is_null() {
            return Vec::new();
        }
        std::slice::from_raw_parts(ptr, len as usize).to_vec()
    }

    unsafe fn make_dict(keys: &[*const c_void], vals: &[*const c_void]) -> Option<CfGuard> {
        debug_assert_eq!(keys.len(), vals.len());
        let p = CFDictionaryCreate(
            kCFAllocatorDefault,
            keys.as_ptr(),
            vals.as_ptr(),
            keys.len() as isize,
            std::ptr::addr_of!(kCFTypeDictionaryKeyCallBacks) as *const u8,
            std::ptr::addr_of!(kCFTypeDictionaryValueCallBacks) as *const u8,
        );
        (!p.is_null()).then_some(CfGuard(p))
    }

    unsafe fn make_cf_i32(v: i32) -> Option<CfGuard> {
        let p = CFNumberCreate(kCFAllocatorDefault, K_CF_NUMBER_SINT32_TYPE, &v as *const i32 as *const c_void);
        (!p.is_null()).then_some(CfGuard(p))
    }

    pub fn generate_random_key_external(is_rsa: bool, bits: u32) -> Result<Vec<u8>, i32> {
        unsafe {
            let num = make_cf_i32(bits as i32).ok_or(STATUS_INVALID_PARAMETER)?;
            let key_type = if is_rsa { kSecAttrKeyTypeRSA } else { kSecAttrKeyTypeECSECPrimeRandom };
            let keys = [kSecAttrKeyType, kSecAttrKeySizeInBits, kSecAttrIsPermanent];
            let vals = [key_type, num.0, kCFBooleanFalse];
            let params = make_dict(&keys, &vals).ok_or(STATUS_INVALID_PARAMETER)?;

            let mut err: *const c_void = std::ptr::null();
            let key_ref = SecKeyCreateRandomKey(params.0, &mut err);
            let _err_guard = CfGuard(err);
            if key_ref.is_null() {
                return Err(STATUS_INVALID_PARAMETER);
            }
            let key_guard = CfGuard(key_ref);

            let mut err2: *const c_void = std::ptr::null();
            let ext = SecKeyCopyExternalRepresentation(key_guard.0, &mut err2);
            let _err2_guard = CfGuard(err2);
            if ext.is_null() {
                return Err(STATUS_INVALID_PARAMETER);
            }
            let ext_guard = CfGuard(ext);
            Ok(cf_data_to_vec(ext_guard.0))
        }
    }

    unsafe fn load_key(is_rsa: bool, is_private: bool, bits: u32, data: &[u8]) -> Result<CfGuard, i32> {
        let cf_data = make_cf_data(data).ok_or(STATUS_INVALID_PARAMETER)?;
        let num = make_cf_i32(bits as i32).ok_or(STATUS_INVALID_PARAMETER)?;
        let key_type = if is_rsa { kSecAttrKeyTypeRSA } else { kSecAttrKeyTypeECSECPrimeRandom };
        let key_class = if is_private { kSecAttrKeyClassPrivate } else { kSecAttrKeyClassPublic };
        let keys = [kSecAttrKeyType, kSecAttrKeyClass, kSecAttrKeySizeInBits];
        let vals = [key_type, key_class, num.0];
        let attrs = make_dict(&keys, &vals).ok_or(STATUS_INVALID_PARAMETER)?;

        let mut err: *const c_void = std::ptr::null();
        let key_ref = SecKeyCreateWithData(cf_data.0, attrs.0, &mut err);
        let _err_guard = CfGuard(err);
        if key_ref.is_null() {
            return Err(STATUS_INVALID_PARAMETER);
        }
        Ok(CfGuard(key_ref))
    }

    pub fn ecdsa_sign_der(bits: u32, x963_priv: &[u8], digest: &[u8]) -> Result<Vec<u8>, i32> {
        unsafe {
            let key = load_key(false, true, bits, x963_priv)?;
            let data = make_cf_data(digest).ok_or(STATUS_INVALID_PARAMETER)?;
            let mut err: *const c_void = std::ptr::null();
            let sig = SecKeyCreateSignature(key.0, kSecKeyAlgorithmECDSASignatureDigestX962, data.0, &mut err);
            let _err_guard = CfGuard(err);
            if sig.is_null() {
                return Err(STATUS_INVALID_PARAMETER);
            }
            let sig_guard = CfGuard(sig);
            Ok(cf_data_to_vec(sig_guard.0))
        }
    }

    pub fn ecdsa_verify_der(bits: u32, x963_pub: &[u8], digest: &[u8], sig_der: &[u8]) -> Result<bool, i32> {
        unsafe {
            let key = load_key(false, false, bits, x963_pub)?;
            let data = make_cf_data(digest).ok_or(STATUS_INVALID_PARAMETER)?;
            let sig = make_cf_data(sig_der).ok_or(STATUS_INVALID_PARAMETER)?;
            let mut err: *const c_void = std::ptr::null();
            let ok = SecKeyVerifySignature(key.0, kSecKeyAlgorithmECDSASignatureDigestX962, data.0, sig.0, &mut err);
            let _err_guard = CfGuard(err);
            Ok(ok != 0)
        }
    }

    pub fn rsa_raw_sign(bits: u32, pkcs1_priv_der: &[u8], em: &[u8]) -> Result<Vec<u8>, i32> {
        unsafe {
            let key = load_key(true, true, bits, pkcs1_priv_der)?;
            let data = make_cf_data(em).ok_or(STATUS_INVALID_PARAMETER)?;
            let mut err: *const c_void = std::ptr::null();
            let sig = SecKeyCreateSignature(key.0, kSecKeyAlgorithmRSASignatureRaw, data.0, &mut err);
            let _err_guard = CfGuard(err);
            if sig.is_null() {
                return Err(STATUS_INVALID_PARAMETER);
            }
            let sig_guard = CfGuard(sig);
            Ok(cf_data_to_vec(sig_guard.0))
        }
    }

    pub fn ecdh_shared_secret(bits: u32, x963_priv: &[u8], x963_pub: &[u8]) -> Result<Vec<u8>, i32> {
        unsafe {
            let priv_key = load_key(false, true, bits, x963_priv)?;
            let pub_key = load_key(false, false, bits, x963_pub)?;
            let params = make_dict(&[], &[]).ok_or(STATUS_INVALID_PARAMETER)?;
            let mut err: *const c_void = std::ptr::null();
            let secret = SecKeyCopyKeyExchangeResult(
                priv_key.0,
                kSecKeyAlgorithmECDHKeyExchangeStandard,
                pub_key.0,
                params.0,
                &mut err,
            );
            let _err_guard = CfGuard(err);
            if secret.is_null() {
                return Err(STATUS_INVALID_PARAMETER);
            }
            let secret_guard = CfGuard(secret);
            Ok(cf_data_to_vec(secret_guard.0))
        }
    }
}

// ---------------------------------------------------------------------------
// RSA keys (public & full private, PKCS#1 v1.5 and PSS sign + verify)
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct RsaPublicKey {
    pub n: Vec<u8>, // big-endian
    pub e: Vec<u8>,
}

#[derive(Clone)]
struct RsaPrivateParams {
    p: Vec<u8>,
    q: Vec<u8>,
    dp: Vec<u8>,
    dq: Vec<u8>,
    qinv: Vec<u8>,
    d: Vec<u8>,
}

#[derive(Clone)]
pub(crate) struct RsaKey {
    pub(crate) pub_key: RsaPublicKey,
    priv_params: Option<RsaPrivateParams>,
}

fn mgf1(d: Digest, seed: &[u8], mask_len: usize) -> Vec<u8> {
    let mut mask = Vec::with_capacity(mask_len + d.len());
    let mut counter = 0u32;
    while mask.len() < mask_len {
        mask.extend(d.hash(&[seed, &counter.to_be_bytes()].concat()));
        counter += 1;
    }
    mask.truncate(mask_len);
    mask
}

fn rev_bytes(b: &[u8]) -> Vec<u8> {
    let mut v = b.to_vec();
    v.reverse();
    v
}

const CAPI_PUBLICKEYBLOB: u8 = 0x06;
const CAPI_PRIVATEKEYBLOB: u8 = 0x07;
const CAPI_CUR_BLOB_VERSION: u8 = 0x02;
const CAPI_CALG_RSA_KEYX: u32 = 0x0000_A400;

impl RsaPublicKey {
    /// `BCRYPT_RSAKEY_BLOB` + public exponent + modulus (big-endian).
    pub fn from_blob(blob: &[u8]) -> Option<RsaPublicKey> {
        let rd = |o: usize| blob.get(o..o + 4).map(|b| u32::from_le_bytes(b.try_into().unwrap()));
        if rd(0)? != BCRYPT_RSAPUBLIC_MAGIC {
            return None;
        }
        let (cb_exp, cb_mod) = (rd(8)? as usize, rd(12)? as usize);
        if cb_exp == 0 || cb_mod == 0 {
            return None;
        }
        let e = blob.get(24..24 + cb_exp)?.to_vec();
        let n = blob.get(24 + cb_exp..24 + cb_exp + cb_mod)?.to_vec();
        Some(RsaPublicKey { n, e })
    }

    fn bits(&self) -> usize {
        bn_bits(&bn_from_be(&self.n))
    }

    fn k(&self) -> usize {
        self.bits().div_ceil(8)
    }

    /// RSAVP1: the encoded message `s^e mod n` (k bytes), or None if s >= n.
    fn open(&self, sig: &[u8]) -> Option<Vec<u8>> {
        let n = bn_from_be(&self.n);
        let s = bn_from_be(sig);
        if n.is_empty() || sig.len() > self.k() || bn_cmp(&s, &n) != std::cmp::Ordering::Less {
            return None;
        }
        Some(bn_to_be(&bn_modexp(&s, &bn_from_be(&self.e), &n), self.k()))
    }

    fn encode_pkcs1_em(&self, digest: Option<Digest>, hash: &[u8]) -> Option<Vec<u8>> {
        let t: Vec<u8> = match digest {
            Some(d) => [d.digest_info_prefix(), hash].concat(),
            None => hash.to_vec(),
        };
        let k = self.k();
        if k < t.len() + 11 {
            return None;
        }
        let mut em = vec![0xFFu8; k];
        em[0] = 0;
        em[1] = 1;
        em[k - t.len() - 1] = 0;
        em[k - t.len()..].copy_from_slice(&t);
        Some(em)
    }

    fn encode_pss_em(&self, d: Digest, hash: &[u8], salt_len: usize) -> Option<Vec<u8>> {
        let mod_bits = self.bits();
        if mod_bits < 16 {
            return None;
        }
        let em_bits = mod_bits - 1;
        let em_len = em_bits.div_ceil(8);
        let k = self.k();
        let h_len = d.len();
        if hash.len() != h_len || em_len < h_len + salt_len + 2 {
            return None;
        }
        let mut salt = vec![0u8; salt_len];
        fill_random(&mut salt);
        let m_prime = [&[0u8; 8][..], hash, &salt].concat();
        let h = d.hash(&m_prime);

        let ps_len = em_len - salt_len - h_len - 2;
        let mut db = Vec::with_capacity(em_len - h_len - 1);
        db.resize(ps_len, 0);
        db.push(0x01);
        db.extend_from_slice(&salt);

        let mut mask = Vec::with_capacity(db.len() + h_len);
        let mut counter = 0u32;
        while mask.len() < db.len() {
            mask.extend(d.hash(&[h.as_slice(), &counter.to_be_bytes()].concat()));
            counter += 1;
        }
        let mut masked_db: Vec<u8> = db.iter().zip(&mask).map(|(a, b)| a ^ b).collect();
        let unused_bits = 8 * em_len - em_bits;
        masked_db[0] &= 0xFFu8 >> unused_bits;

        let mut em_full = vec![0u8; k.saturating_sub(em_len)];
        em_full.extend_from_slice(&masked_db);
        em_full.extend_from_slice(&h);
        em_full.push(0xBC);
        Some(em_full)
    }

    /// EMSA-PKCS1-v1_5. `digest` None = no DigestInfo (raw hash, pszAlgId NULL).
    pub fn verify_pkcs1(&self, digest: Option<Digest>, hash: &[u8], sig: &[u8]) -> bool {
        let Some(em) = self.open(sig) else { return false };
        let Some(expect) = self.encode_pkcs1_em(digest, hash) else { return false };
        em == expect
    }

    /// EMSA-PSS-VERIFY with MGF1 using the same digest.
    pub fn verify_pss(&self, d: Digest, hash: &[u8], salt_len: usize, sig: &[u8]) -> bool {
        let Some(em_full) = self.open(sig) else { return false };
        let mod_bits = self.bits();
        if mod_bits < 16 {
            return false;
        }
        let em_bits = mod_bits - 1;
        let em_len = em_bits.div_ceil(8);
        // EM is the low em_len bytes of the k-byte RSAVP1 output.
        let em = &em_full[em_full.len() - em_len..];
        if em_full[..em_full.len() - em_len].iter().any(|&b| b != 0) {
            return false;
        }
        let h_len = d.len();
        if hash.len() != h_len || em_len < h_len + salt_len + 2 || em[em_len - 1] != 0xBC {
            return false;
        }
        let (masked_db, h) = em[..em_len - 1].split_at(em_len - h_len - 1);
        let unused_bits = 8 * em_len - em_bits;
        if masked_db[0] & !(0xFFu8 >> unused_bits) != 0 {
            return false;
        }
        // MGF1(H, len(masked_db))
        let mut mask = Vec::with_capacity(masked_db.len() + h_len);
        let mut counter = 0u32;
        while mask.len() < masked_db.len() {
            mask.extend(d.hash(&[h, &counter.to_be_bytes()].concat()));
            counter += 1;
        }
        let mut db: Vec<u8> = masked_db.iter().zip(&mask).map(|(a, b)| a ^ b).collect();
        db[0] &= 0xFF >> unused_bits;
        let ps_len = em_len - h_len - salt_len - 2;
        if db[..ps_len].iter().any(|&b| b != 0) || db[ps_len] != 1 {
            return false;
        }
        let salt = &db[db.len() - salt_len..];
        let m_prime = [&[0u8; 8][..], hash, salt].concat();
        d.hash(&m_prime) == h
    }
}

impl RsaKey {
    pub(crate) fn generate(bits: u32) -> Result<Self, i32> {
        let der = seckey::generate_random_key_external(true, bits)?;
        let ints = parse_der_seq_uints(&der).ok_or(STATUS_INVALID_PARAMETER)?;
        // PKCS#1 RSAPrivateKey: [version, n, e, d, p, q, dp, dq, qinv]
        if ints.len() < 9 {
            return Err(STATUS_INVALID_PARAMETER);
        }
        let k = (bits as usize).div_ceil(8);
        let p_len = k.div_ceil(2);
        let n = pad_be(&ints[1], k).ok_or(STATUS_INVALID_PARAMETER)?;
        let e = strip_leading_zeros(&ints[2]).to_vec();
        let d = pad_be(&ints[3], k).ok_or(STATUS_INVALID_PARAMETER)?;
        let p = pad_be(&ints[4], p_len).unwrap_or_else(|| ints[4].clone());
        let q = pad_be(&ints[5], p_len).unwrap_or_else(|| ints[5].clone());
        let dp = pad_be(&ints[6], p.len()).unwrap_or_else(|| ints[6].clone());
        let dq = pad_be(&ints[7], q.len()).unwrap_or_else(|| ints[7].clone());
        let qinv = pad_be(&ints[8], p.len()).unwrap_or_else(|| ints[8].clone());
        Ok(RsaKey {
            pub_key: RsaPublicKey { n, e },
            priv_params: Some(RsaPrivateParams { p, q, dp, dq, qinv, d }),
        })
    }

    pub(crate) fn bits(&self) -> usize {
        self.pub_key.bits()
    }

    pub(crate) fn k(&self) -> usize {
        self.pub_key.k()
    }

    pub(crate) fn has_private(&self) -> bool {
        self.priv_params.is_some()
    }

    /// Parse a legacy CryptoAPI `PUBLICKEYBLOB` (0x06) or `PRIVATEKEYBLOB` (0x07).
    /// All big integers in CAPI blobs are little-endian.
    pub(crate) fn from_capi_blob(blob: &[u8]) -> Option<(Self, u32, bool)> {
        if blob.len() < 20 {
            return None;
        }
        let b_type = blob[0];
        let b_ver = blob[1];
        if b_ver != CAPI_CUR_BLOB_VERSION {
            return None;
        }
        let ai_key_alg = u32::from_le_bytes(blob[4..8].try_into().unwrap());
        let magic = u32::from_le_bytes(blob[8..12].try_into().unwrap());
        let bitlen = u32::from_le_bytes(blob[12..16].try_into().unwrap()) as usize;
        let pubexp = u32::from_le_bytes(blob[16..20].try_into().unwrap());
        if bitlen < 64 || bitlen % 8 != 0 || pubexp == 0 {
            return None;
        }
        let k = bitlen / 8;
        let half = k / 2;
        let e = strip_leading_zeros(&pubexp.to_be_bytes()).to_vec();
        match (b_type, magic) {
            (CAPI_PUBLICKEYBLOB, BCRYPT_RSAPUBLIC_MAGIC) => {
                if blob.len() < 20 + k {
                    return None;
                }
                let n = rev_bytes(&blob[20..20 + k]);
                Some((RsaKey { pub_key: RsaPublicKey { n, e }, priv_params: None }, ai_key_alg, false))
            }
            (CAPI_PRIVATEKEYBLOB, BCRYPT_RSAPRIVATE_MAGIC) => {
                if k % 2 != 0 || blob.len() < 20 + 2 * k + 5 * half {
                    return None;
                }
                let mut off = 20;
                let n = rev_bytes(&blob[off..off + k]);
                off += k;
                let p = rev_bytes(&blob[off..off + half]);
                off += half;
                let q = rev_bytes(&blob[off..off + half]);
                off += half;
                let dp = rev_bytes(&blob[off..off + half]);
                off += half;
                let dq = rev_bytes(&blob[off..off + half]);
                off += half;
                let qinv = rev_bytes(&blob[off..off + half]);
                off += half;
                let d = rev_bytes(&blob[off..off + k]);
                Some((
                    RsaKey {
                        pub_key: RsaPublicKey { n, e },
                        priv_params: Some(RsaPrivateParams { p, q, dp, dq, qinv, d }),
                    },
                    ai_key_alg,
                    true,
                ))
            }
            _ => None,
        }
    }

    pub(crate) fn to_capi_public_blob(&self, ai_key_alg: u32) -> Option<Vec<u8>> {
        let e_trim = strip_leading_zeros(&self.pub_key.e);
        if e_trim.len() > 4 {
            return None;
        }
        let mut e_arr = [0u8; 4];
        e_arr[4 - e_trim.len()..].copy_from_slice(e_trim);
        let pubexp = u32::from_be_bytes(e_arr);
        let k = self.k();
        let n_be = pad_be(&self.pub_key.n, k)?;
        let bitlen = (k * 8) as u32;
        let mut out = Vec::with_capacity(20 + k);
        out.push(CAPI_PUBLICKEYBLOB);
        out.push(CAPI_CUR_BLOB_VERSION);
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&ai_key_alg.to_le_bytes());
        out.extend_from_slice(&BCRYPT_RSAPUBLIC_MAGIC.to_le_bytes());
        out.extend_from_slice(&bitlen.to_le_bytes());
        out.extend_from_slice(&pubexp.to_le_bytes());
        out.extend_from_slice(&rev_bytes(&n_be));
        Some(out)
    }

    pub(crate) fn to_capi_private_blob(&self, ai_key_alg: u32) -> Option<Vec<u8>> {
        let priv_p = self.priv_params.as_ref()?;
        let e_trim = strip_leading_zeros(&self.pub_key.e);
        if e_trim.len() > 4 {
            return None;
        }
        let mut e_arr = [0u8; 4];
        e_arr[4 - e_trim.len()..].copy_from_slice(e_trim);
        let pubexp = u32::from_be_bytes(e_arr);
        let k = self.k();
        if k % 2 != 0 {
            return None;
        }
        let half = k / 2;
        let n_be = pad_be(&self.pub_key.n, k)?;
        let p_be = pad_be(&priv_p.p, half)?;
        let q_be = pad_be(&priv_p.q, half)?;
        let dp_be = pad_be(&priv_p.dp, half)?;
        let dq_be = pad_be(&priv_p.dq, half)?;
        let qinv_be = pad_be(&priv_p.qinv, half)?;
        let d_be = pad_be(&priv_p.d, k)?;
        let bitlen = (k * 8) as u32;
        let mut out = Vec::with_capacity(20 + 2 * k + 5 * half);
        out.push(CAPI_PRIVATEKEYBLOB);
        out.push(CAPI_CUR_BLOB_VERSION);
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&ai_key_alg.to_le_bytes());
        out.extend_from_slice(&BCRYPT_RSAPRIVATE_MAGIC.to_le_bytes());
        out.extend_from_slice(&bitlen.to_le_bytes());
        out.extend_from_slice(&pubexp.to_le_bytes());
        out.extend_from_slice(&rev_bytes(&n_be));
        out.extend_from_slice(&rev_bytes(&p_be));
        out.extend_from_slice(&rev_bytes(&q_be));
        out.extend_from_slice(&rev_bytes(&dp_be));
        out.extend_from_slice(&rev_bytes(&dq_be));
        out.extend_from_slice(&rev_bytes(&qinv_be));
        out.extend_from_slice(&rev_bytes(&d_be));
        Some(out)
    }

    fn from_blob(blob_type: &str, blob: &[u8]) -> Option<Self> {
        if matches!(blob_type, "CAPIPUBLICBLOB" | "CAPIPRIVATEBLOB") {
            let (rsa, _, is_priv) = Self::from_capi_blob(blob)?;
            return match (blob_type, is_priv) {
                ("CAPIPUBLICBLOB", false) | ("CAPIPRIVATEBLOB", true) => Some(rsa),
                _ => None,
            };
        }
        let rd = |o: usize| blob.get(o..o + 4).map(|b| u32::from_le_bytes(b.try_into().unwrap()));
        let magic = rd(0)?;
        let cb_exp = rd(8)? as usize;
        let cb_mod = rd(12)? as usize;
        let cb_p1 = rd(16)? as usize;
        let cb_p2 = rd(20)? as usize;
        if cb_exp == 0 || cb_mod == 0 {
            return None;
        }
        match (blob_type, magic) {
            ("RSAPUBLICBLOB" | "PUBLICBLOB", BCRYPT_RSAPUBLIC_MAGIC) => {
                let pub_key = RsaPublicKey::from_blob(blob)?;
                Some(RsaKey { pub_key, priv_params: None })
            }
            ("RSAPRIVATEBLOB" | "PRIVATEBLOB", BCRYPT_RSAPRIVATE_MAGIC) => {
                if cb_p1 == 0 || cb_p2 == 0 {
                    return None;
                }
                let mut off = 24;
                let e = blob.get(off..off + cb_exp)?.to_vec();
                off += cb_exp;
                let n = blob.get(off..off + cb_mod)?.to_vec();
                off += cb_mod;
                let p = blob.get(off..off + cb_p1)?.to_vec();
                off += cb_p1;
                let q = blob.get(off..off + cb_p2)?.to_vec();
                // Compute CRT parameters (d, dp, dq, qinv) from (e, p, q).
                let e_bn = bn_from_be(&e);
                let p_bn = bn_from_be(&p);
                let q_bn = bn_from_be(&q);
                if p_bn.is_empty() || q_bn.is_empty() || e_bn.is_empty() {
                    return None;
                }
                let p1 = bn_sub(&p_bn, &[1]);
                let q1 = bn_sub(&q_bn, &[1]);
                let mut phi = bn_mul(&p1, &q1);
                bn_trim(&mut phi);
                let d_bn = bn_modinv(&e_bn, &phi)?;
                let dp_bn = bn_mod(&d_bn, &p1);
                let dq_bn = bn_mod(&d_bn, &q1);
                let qinv_bn = bn_modinv(&q_bn, &p_bn)?;
                Some(RsaKey {
                    pub_key: RsaPublicKey { n, e },
                    priv_params: Some(RsaPrivateParams {
                        dp: bn_to_be(&dp_bn, cb_p1),
                        dq: bn_to_be(&dq_bn, cb_p2),
                        qinv: bn_to_be(&qinv_bn, cb_p1),
                        d: bn_to_be(&d_bn, cb_mod),
                        p,
                        q,
                    }),
                })
            }
            ("RSAFULLPRIVATEBLOB", BCRYPT_RSAFULLPRIVATE_MAGIC) => {
                if cb_p1 == 0 || cb_p2 == 0 {
                    return None;
                }
                let mut off = 24;
                let e = blob.get(off..off + cb_exp)?.to_vec();
                off += cb_exp;
                let n = blob.get(off..off + cb_mod)?.to_vec();
                off += cb_mod;
                let p = blob.get(off..off + cb_p1)?.to_vec();
                off += cb_p1;
                let q = blob.get(off..off + cb_p2)?.to_vec();
                off += cb_p2;
                let dp = blob.get(off..off + cb_p1)?.to_vec();
                off += cb_p1;
                let dq = blob.get(off..off + cb_p2)?.to_vec();
                off += cb_p2;
                let qinv = blob.get(off..off + cb_p1)?.to_vec();
                off += cb_p1;
                let d = blob.get(off..off + cb_mod)?.to_vec();
                Some(RsaKey {
                    pub_key: RsaPublicKey { n, e },
                    priv_params: Some(RsaPrivateParams { p, q, dp, dq, qinv, d }),
                })
            }
            _ => None,
        }
    }

    fn to_public_blob(&self) -> Vec<u8> {
        let bits = self.pub_key.bits() as u32;
        let mut b = Vec::with_capacity(24 + self.pub_key.e.len() + self.pub_key.n.len());
        for v in [
            BCRYPT_RSAPUBLIC_MAGIC,
            bits,
            self.pub_key.e.len() as u32,
            self.pub_key.n.len() as u32,
            0,
            0,
        ] {
            b.extend_from_slice(&v.to_le_bytes());
        }
        b.extend_from_slice(&self.pub_key.e);
        b.extend_from_slice(&self.pub_key.n);
        b
    }

    fn to_private_blob(&self, full: bool) -> Option<Vec<u8>> {
        let priv_p = self.priv_params.as_ref()?;
        let bits = self.pub_key.bits() as u32;
        let magic = if full { BCRYPT_RSAFULLPRIVATE_MAGIC } else { BCRYPT_RSAPRIVATE_MAGIC };
        let mut b = Vec::new();
        for v in [
            magic,
            bits,
            self.pub_key.e.len() as u32,
            self.pub_key.n.len() as u32,
            priv_p.p.len() as u32,
            priv_p.q.len() as u32,
        ] {
            b.extend_from_slice(&v.to_le_bytes());
        }
        b.extend_from_slice(&self.pub_key.e);
        b.extend_from_slice(&self.pub_key.n);
        b.extend_from_slice(&priv_p.p);
        b.extend_from_slice(&priv_p.q);
        if full {
            b.extend_from_slice(&pad_be(&priv_p.dp, priv_p.p.len())?);
            b.extend_from_slice(&pad_be(&priv_p.dq, priv_p.q.len())?);
            b.extend_from_slice(&pad_be(&priv_p.qinv, priv_p.p.len())?);
            b.extend_from_slice(&pad_be(&priv_p.d, self.pub_key.n.len())?);
        }
        Some(b)
    }

    fn to_pkcs1_private_der(&self) -> Option<Vec<u8>> {
        let priv_p = self.priv_params.as_ref()?;
        let mut body = Vec::new();
        body.extend_from_slice(&der_uint(&[0]));
        body.extend_from_slice(&der_uint(&self.pub_key.n));
        body.extend_from_slice(&der_uint(&self.pub_key.e));
        body.extend_from_slice(&der_uint(&priv_p.d));
        body.extend_from_slice(&der_uint(&priv_p.p));
        body.extend_from_slice(&der_uint(&priv_p.q));
        body.extend_from_slice(&der_uint(&priv_p.dp));
        body.extend_from_slice(&der_uint(&priv_p.dq));
        body.extend_from_slice(&der_uint(&priv_p.qinv));
        Some(der_tlv(0x30, &body))
    }

    /// RSADP / RSASP1: `m^d mod n` (k bytes). Uses Apple `SecKey` hardware/accelerate path
    /// when available (>= 1024-bit keys) and falls back to big-integer CRT for small keys.
    fn private_raw_op(&self, input: &[u8]) -> Result<Vec<u8>, i32> {
        let priv_p = self.priv_params.as_ref().ok_or(STATUS_INVALID_HANDLE)?;
        let k = self.pub_key.k();
        if input.len() != k {
            return Err(STATUS_INVALID_PARAMETER);
        }
        let n_bn = bn_from_be(&self.pub_key.n);
        let c_bn = bn_from_be(input);
        if bn_cmp(&c_bn, &n_bn) != std::cmp::Ordering::Less {
            return Err(STATUS_INVALID_PARAMETER);
        }
        if self.pub_key.bits() >= 1024 {
            if let Some(der) = self.to_pkcs1_private_der() {
                if let Ok(sig) = seckey::rsa_raw_sign(self.pub_key.bits() as u32, &der, input) {
                    if let Some(padded) = pad_be(&sig, k) {
                        return Ok(padded);
                    }
                }
            }
        }
        // Pure-Rust CRT fallback (supports 512-bit / non-standard key sizes too).
        let p_bn = bn_from_be(&priv_p.p);
        let q_bn = bn_from_be(&priv_p.q);
        let dp_bn = bn_from_be(&priv_p.dp);
        let dq_bn = bn_from_be(&priv_p.dq);
        let qinv_bn = bn_from_be(&priv_p.qinv);
        if !p_bn.is_empty() && !q_bn.is_empty() && !dp_bn.is_empty() && !dq_bn.is_empty() && !qinv_bn.is_empty() {
            let m1 = bn_modexp(&c_bn, &dp_bn, &p_bn);
            let m2 = bn_modexp(&c_bn, &dq_bn, &q_bn);
            let m2_mod_p = bn_mod(&m2, &p_bn);
            let diff = if bn_cmp(&m1, &m2_mod_p) != std::cmp::Ordering::Less {
                bn_sub(&m1, &m2_mod_p)
            } else {
                bn_sub(&bn_add(&m1, &p_bn), &m2_mod_p)
            };
            let h = bn_mod(&bn_mul(&qinv_bn, &diff), &p_bn);
            let mut m = bn_add(&m2, &bn_mul(&h, &q_bn));
            bn_trim(&mut m);
            Ok(bn_to_be(&m, k))
        } else {
            let d_bn = bn_from_be(&priv_p.d);
            Ok(bn_to_be(&bn_modexp(&c_bn, &d_bn, &n_bn), k))
        }
    }

    pub(crate) fn encrypt_raw(&self, plaintext: &[u8]) -> Result<Vec<u8>, i32> {
        if plaintext.len() != self.pub_key.k() {
            return Err(STATUS_INVALID_PARAMETER);
        }
        self.pub_key.open(plaintext).ok_or(STATUS_INVALID_PARAMETER)
    }

    pub(crate) fn decrypt_raw(&self, ciphertext: &[u8]) -> Result<Vec<u8>, i32> {
        self.private_raw_op(ciphertext)
    }

    /// RSAES-PKCS1-v1_5 encrypt (RFC 8017 Section 7.2.1).
    pub(crate) fn encrypt_pkcs1(&self, plaintext: &[u8]) -> Result<Vec<u8>, i32> {
        let k = self.pub_key.k();
        if k < 11 || plaintext.len() > k - 11 {
            return Err(STATUS_INVALID_PARAMETER);
        }
        let ps_len = k - plaintext.len() - 3;
        let mut em = Vec::with_capacity(k);
        em.push(0x00);
        em.push(0x02);
        // Generate ps_len non-zero random padding bytes.
        let mut ps = vec![0u8; ps_len];
        for b in ps.iter_mut() {
            let mut r = [0u8; 1];
            loop {
                fill_random(&mut r);
                if r[0] != 0 {
                    *b = r[0];
                    break;
                }
            }
        }
        em.extend_from_slice(&ps);
        em.push(0x00);
        em.extend_from_slice(plaintext);
        self.pub_key.open(&em).ok_or(STATUS_INVALID_PARAMETER)
    }

    /// RSAES-PKCS1-v1_5 decrypt (RFC 8017 Section 7.2.2).
    pub(crate) fn decrypt_pkcs1(&self, ciphertext: &[u8]) -> Result<Vec<u8>, i32> {
        let em = self.private_raw_op(ciphertext)?;
        if em.len() < 11 || em[0] != 0x00 || em[1] != 0x02 {
            return Err(STATUS_DATA_ERROR);
        }
        let sep = em[2..].iter().position(|&b| b == 0x00).ok_or(STATUS_DATA_ERROR)?;
        if sep < 8 {
            return Err(STATUS_DATA_ERROR);
        }
        Ok(em[2 + sep + 1..].to_vec())
    }

    /// RSAES-OAEP encrypt (RFC 8017 Section 7.1.1) with MGF1 using `d`.
    pub(crate) fn encrypt_oaep(&self, d: Digest, label: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, i32> {
        let k = self.pub_key.k();
        let h_len = d.len();
        if k < 2 * h_len + 2 || plaintext.len() > k - 2 * h_len - 2 {
            return Err(STATUS_INVALID_PARAMETER);
        }
        let l_hash = d.hash(label);
        let ps_len = k - plaintext.len() - 2 * h_len - 2;
        let mut db = Vec::with_capacity(k - h_len - 1);
        db.extend_from_slice(&l_hash);
        db.resize(db.len() + ps_len, 0x00);
        db.push(0x01);
        db.extend_from_slice(plaintext);

        let mut seed = vec![0u8; h_len];
        fill_random(&mut seed);
        let db_mask = mgf1(d, &seed, db.len());
        let masked_db: Vec<u8> = db.iter().zip(&db_mask).map(|(a, b)| a ^ b).collect();
        let seed_mask = mgf1(d, &masked_db, h_len);
        let masked_seed: Vec<u8> = seed.iter().zip(&seed_mask).map(|(a, b)| a ^ b).collect();

        let mut em = Vec::with_capacity(k);
        em.push(0x00);
        em.extend_from_slice(&masked_seed);
        em.extend_from_slice(&masked_db);
        self.pub_key.open(&em).ok_or(STATUS_INVALID_PARAMETER)
    }

    /// RSAES-OAEP decrypt (RFC 8017 Section 7.1.2) with MGF1 using `d`.
    pub(crate) fn decrypt_oaep(&self, d: Digest, label: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, i32> {
        let k = self.pub_key.k();
        let h_len = d.len();
        if k < 2 * h_len + 2 || ciphertext.len() != k {
            return Err(STATUS_INVALID_PARAMETER);
        }
        let em = self.private_raw_op(ciphertext)?;
        let y = em[0];
        let masked_seed = &em[1..1 + h_len];
        let masked_db = &em[1 + h_len..];
        let seed_mask = mgf1(d, masked_db, h_len);
        let seed: Vec<u8> = masked_seed.iter().zip(&seed_mask).map(|(a, b)| a ^ b).collect();
        let db_mask = mgf1(d, &seed, masked_db.len());
        let db: Vec<u8> = masked_db.iter().zip(&db_mask).map(|(a, b)| a ^ b).collect();

        let l_hash = d.hash(label);
        let mut bad = y;
        for (&a, &b) in db[..h_len].iter().zip(&l_hash) {
            bad |= a ^ b;
        }
        let mut found_one = false;
        let mut msg_start = 0usize;
        for (idx, &b) in db[h_len..].iter().enumerate() {
            if !found_one {
                if b == 0x01 {
                    found_one = true;
                    msg_start = h_len + idx + 1;
                } else if b != 0x00 {
                    bad |= 1;
                }
            }
        }
        if bad != 0 || !found_one {
            return Err(STATUS_DATA_ERROR);
        }
        Ok(db[msg_start..].to_vec())
    }

    fn sign_raw_em(&self, em: &[u8]) -> Result<Vec<u8>, i32> {
        self.private_raw_op(em)
    }

    pub(crate) fn sign_pkcs1(&self, digest: Option<Digest>, hash: &[u8]) -> Result<Vec<u8>, i32> {
        let em = self.pub_key.encode_pkcs1_em(digest, hash).ok_or(STATUS_INVALID_PARAMETER)?;
        self.sign_raw_em(&em)
    }

    pub(crate) fn sign_pss(&self, d: Digest, hash: &[u8], salt_len: usize) -> Result<Vec<u8>, i32> {
        let em = self.pub_key.encode_pss_em(d, hash, salt_len).ok_or(STATUS_INVALID_PARAMETER)?;
        self.sign_raw_em(&em)
    }

    pub(crate) fn verify_pkcs1(&self, digest: Option<Digest>, hash: &[u8], sig: &[u8]) -> bool {
        self.pub_key.verify_pkcs1(digest, hash, sig)
    }
}

// ---------------------------------------------------------------------------
// ECDSA & ECDH P-256 / P-384 / P-521 (Apple Security.framework SecKey backend)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EccUsage {
    Ecdsa,
    Ecdh,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EccCurve {
    P256,
    P384,
    P521,
}

impl EccCurve {
    fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            256 => Some(Self::P256),
            384 => Some(Self::P384),
            521 => Some(Self::P521),
            _ => None,
        }
    }

    fn from_win_curve_name(s: &str) -> Option<Self> {
        match s.trim_end_matches('\0').to_ascii_lowercase().as_str() {
            "nistp256" | "secp256r1" | "p256" => Some(Self::P256),
            "nistp384" | "secp384r1" | "p384" => Some(Self::P384),
            "nistp521" | "secp521r1" | "p521" => Some(Self::P521),
            _ => None,
        }
    }

    fn bits(self) -> u32 {
        match self {
            Self::P256 => 256,
            Self::P384 => 384,
            Self::P521 => 521,
        }
    }

    fn coord_len(self) -> usize {
        match self {
            Self::P256 => 32,
            Self::P384 => 48,
            Self::P521 => 66,
        }
    }

    fn pub_magic(self, usage: EccUsage) -> u32 {
        match (usage, self) {
            (EccUsage::Ecdsa, Self::P256) => BCRYPT_ECDSA_PUBLIC_P256_MAGIC,
            (EccUsage::Ecdsa, Self::P384) => BCRYPT_ECDSA_PUBLIC_P384_MAGIC,
            (EccUsage::Ecdsa, Self::P521) => BCRYPT_ECDSA_PUBLIC_P521_MAGIC,
            (EccUsage::Ecdh, Self::P256) => BCRYPT_ECDH_PUBLIC_P256_MAGIC,
            (EccUsage::Ecdh, Self::P384) => BCRYPT_ECDH_PUBLIC_P384_MAGIC,
            (EccUsage::Ecdh, Self::P521) => BCRYPT_ECDH_PUBLIC_P521_MAGIC,
        }
    }

    fn priv_magic(self, usage: EccUsage) -> u32 {
        match (usage, self) {
            (EccUsage::Ecdsa, Self::P256) => BCRYPT_ECDSA_PRIVATE_P256_MAGIC,
            (EccUsage::Ecdsa, Self::P384) => BCRYPT_ECDSA_PRIVATE_P384_MAGIC,
            (EccUsage::Ecdsa, Self::P521) => BCRYPT_ECDSA_PRIVATE_P521_MAGIC,
            (EccUsage::Ecdh, Self::P256) => BCRYPT_ECDH_PRIVATE_P256_MAGIC,
            (EccUsage::Ecdh, Self::P384) => BCRYPT_ECDH_PRIVATE_P384_MAGIC,
            (EccUsage::Ecdh, Self::P521) => BCRYPT_ECDH_PRIVATE_P521_MAGIC,
        }
    }

    fn win_curve_name(self) -> &'static str {
        match self {
            Self::P256 => "nistP256",
            Self::P384 => "nistP384",
            Self::P521 => "nistP521",
        }
    }

    fn alg_name(self, usage: EccUsage) -> &'static str {
        match (usage, self) {
            (EccUsage::Ecdsa, Self::P256) => "ECDSA_P256",
            (EccUsage::Ecdsa, Self::P384) => "ECDSA_P384",
            (EccUsage::Ecdsa, Self::P521) => "ECDSA_P521",
            (EccUsage::Ecdh, Self::P256) => "ECDH_P256",
            (EccUsage::Ecdh, Self::P384) => "ECDH_P384",
            (EccUsage::Ecdh, Self::P521) => "ECDH_P521",
        }
    }
}

#[derive(Clone)]
struct EccKey {
    usage: EccUsage,
    curve: EccCurve,
    x: Vec<u8>,
    y: Vec<u8>,
    d: Option<Vec<u8>>,
}

impl EccKey {
    fn generate(usage: EccUsage, curve: EccCurve) -> Result<Self, i32> {
        let ext = seckey::generate_random_key_external(false, curve.bits())?;
        let cl = curve.coord_len();
        // Apple X9.63 private representation: 0x04 || X (cl) || Y (cl) || d (cl)
        if ext.len() != 1 + 3 * cl || ext[0] != 0x04 {
            return Err(STATUS_INVALID_PARAMETER);
        }
        Ok(EccKey {
            usage,
            curve,
            x: ext[1..1 + cl].to_vec(),
            y: ext[1 + cl..1 + 2 * cl].to_vec(),
            d: Some(ext[1 + 2 * cl..1 + 3 * cl].to_vec()),
        })
    }

    fn from_blob(blob_type: &str, expected_usage: EccUsage, expected_curve: Option<EccCurve>, blob: &[u8]) -> Option<Self> {
        if blob.len() < 8 {
            return None;
        }
        let magic = u32::from_le_bytes(blob[0..4].try_into().unwrap());
        let cb_key = u32::from_le_bytes(blob[4..8].try_into().unwrap()) as usize;
        let curve_from_len = || match cb_key {
            32 => Some(EccCurve::P256),
            48 => Some(EccCurve::P384),
            66 => Some(EccCurve::P521),
            _ => None,
        };
        let (usage, curve, is_priv) = match magic {
            BCRYPT_ECDSA_PUBLIC_P256_MAGIC => (EccUsage::Ecdsa, EccCurve::P256, false),
            BCRYPT_ECDSA_PRIVATE_P256_MAGIC => (EccUsage::Ecdsa, EccCurve::P256, true),
            BCRYPT_ECDSA_PUBLIC_P384_MAGIC => (EccUsage::Ecdsa, EccCurve::P384, false),
            BCRYPT_ECDSA_PRIVATE_P384_MAGIC => (EccUsage::Ecdsa, EccCurve::P384, true),
            BCRYPT_ECDSA_PUBLIC_P521_MAGIC => (EccUsage::Ecdsa, EccCurve::P521, false),
            BCRYPT_ECDSA_PRIVATE_P521_MAGIC => (EccUsage::Ecdsa, EccCurve::P521, true),
            BCRYPT_ECDSA_PUBLIC_GENERIC_MAGIC => (EccUsage::Ecdsa, expected_curve.or_else(curve_from_len)?, false),
            BCRYPT_ECDSA_PRIVATE_GENERIC_MAGIC => (EccUsage::Ecdsa, expected_curve.or_else(curve_from_len)?, true),
            BCRYPT_ECDH_PUBLIC_P256_MAGIC => (EccUsage::Ecdh, EccCurve::P256, false),
            BCRYPT_ECDH_PRIVATE_P256_MAGIC => (EccUsage::Ecdh, EccCurve::P256, true),
            BCRYPT_ECDH_PUBLIC_P384_MAGIC => (EccUsage::Ecdh, EccCurve::P384, false),
            BCRYPT_ECDH_PRIVATE_P384_MAGIC => (EccUsage::Ecdh, EccCurve::P384, true),
            BCRYPT_ECDH_PUBLIC_P521_MAGIC => (EccUsage::Ecdh, EccCurve::P521, false),
            BCRYPT_ECDH_PRIVATE_P521_MAGIC => (EccUsage::Ecdh, EccCurve::P521, true),
            BCRYPT_ECDH_PUBLIC_GENERIC_MAGIC => (EccUsage::Ecdh, expected_curve.or_else(curve_from_len)?, false),
            BCRYPT_ECDH_PRIVATE_GENERIC_MAGIC => (EccUsage::Ecdh, expected_curve.or_else(curve_from_len)?, true),
            _ => return None,
        };
        if usage != expected_usage {
            return None;
        }
        if let Some(exp) = expected_curve {
            if exp != curve {
                return None;
            }
        }
        if cb_key != curve.coord_len() {
            return None;
        }
        match (blob_type, is_priv) {
            ("ECCPUBLICBLOB" | "PUBLICBLOB", false) => {
                if blob.len() < 8 + 2 * cb_key {
                    return None;
                }
                Some(EccKey {
                    usage,
                    curve,
                    x: blob[8..8 + cb_key].to_vec(),
                    y: blob[8 + cb_key..8 + 2 * cb_key].to_vec(),
                    d: None,
                })
            }
            ("ECCPRIVATEBLOB" | "PRIVATEBLOB", true) => {
                if blob.len() < 8 + 3 * cb_key {
                    return None;
                }
                Some(EccKey {
                    usage,
                    curve,
                    x: blob[8..8 + cb_key].to_vec(),
                    y: blob[8 + cb_key..8 + 2 * cb_key].to_vec(),
                    d: Some(blob[8 + 2 * cb_key..8 + 3 * cb_key].to_vec()),
                })
            }
            _ => None,
        }
    }

    fn to_public_blob(&self) -> Vec<u8> {
        let cl = self.curve.coord_len();
        let mut out = Vec::with_capacity(8 + 2 * cl);
        out.extend_from_slice(&self.curve.pub_magic(self.usage).to_le_bytes());
        out.extend_from_slice(&(cl as u32).to_le_bytes());
        out.extend_from_slice(&self.x);
        out.extend_from_slice(&self.y);
        out
    }

    fn to_private_blob(&self) -> Option<Vec<u8>> {
        let d = self.d.as_ref()?;
        let cl = self.curve.coord_len();
        let mut out = Vec::with_capacity(8 + 3 * cl);
        out.extend_from_slice(&self.curve.priv_magic(self.usage).to_le_bytes());
        out.extend_from_slice(&(cl as u32).to_le_bytes());
        out.extend_from_slice(&self.x);
        out.extend_from_slice(&self.y);
        out.extend_from_slice(d);
        Some(out)
    }

    fn x963_public(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(1 + self.x.len() + self.y.len());
        out.push(0x04);
        out.extend_from_slice(&self.x);
        out.extend_from_slice(&self.y);
        out
    }

    fn x963_private(&self) -> Option<Vec<u8>> {
        let d = self.d.as_ref()?;
        let mut out = self.x963_public();
        out.extend_from_slice(d);
        Some(out)
    }

    fn sign_hash(&self, hash: &[u8]) -> Result<Vec<u8>, i32> {
        let priv_bytes = self.x963_private().ok_or(STATUS_INVALID_HANDLE)?;
        let sig_der = seckey::ecdsa_sign_der(self.curve.bits(), &priv_bytes, hash)?;
        let ints = parse_der_seq_uints(&sig_der).ok_or(STATUS_INVALID_PARAMETER)?;
        if ints.len() != 2 {
            return Err(STATUS_INVALID_PARAMETER);
        }
        let cl = self.curve.coord_len();
        let mut out = Vec::with_capacity(2 * cl);
        out.extend_from_slice(&pad_be(&ints[0], cl).ok_or(STATUS_INVALID_PARAMETER)?);
        out.extend_from_slice(&pad_be(&ints[1], cl).ok_or(STATUS_INVALID_PARAMETER)?);
        Ok(out)
    }

    fn verify_hash(&self, hash: &[u8], sig_rs: &[u8]) -> Result<bool, i32> {
        let cl = self.curve.coord_len();
        if sig_rs.len() != 2 * cl {
            return Ok(false);
        }
        let r = &sig_rs[..cl];
        let s = &sig_rs[cl..];
        let mut body = der_uint(r);
        body.extend_from_slice(&der_uint(s));
        let sig_der = der_tlv(0x30, &body);
        seckey::ecdsa_verify_der(self.curve.bits(), &self.x963_public(), hash, &sig_der)
    }

    fn ecdh_agree(&self, peer_pub: &EccKey) -> Result<Vec<u8>, i32> {
        if self.curve != peer_pub.curve {
            return Err(STATUS_INVALID_PARAMETER);
        }
        let priv_bytes = self.x963_private().ok_or(STATUS_INVALID_HANDLE)?;
        let raw = seckey::ecdh_shared_secret(self.curve.bits(), &priv_bytes, &peer_pub.x963_public())?;
        pad_be(&raw, self.curve.coord_len()).ok_or(STATUS_INVALID_PARAMETER)
    }
}

// ---------------------------------------------------------------------------
// Symmetric Ciphers: AES-128/192/256 (ECB/CBC/CFB/CTR/GCM/CCM + Key Wrap + CMAC),
// DES, 3DES, 3DES_112 (CommonCrypto), RC2 (RFC 2268), RC4 (stream cipher)
// ---------------------------------------------------------------------------

const KCC_ENCRYPT: u32 = 0;
const KCC_DECRYPT: u32 = 1;
const KCC_ALGORITHM_AES: u32 = 0;
const KCC_ALGORITHM_DES: u32 = 1;
const KCC_ALGORITHM_3DES: u32 = 2;
const KCC_OPTION_PKCS7_PADDING: u32 = 1;
const KCC_OPTION_ECB_MODE: u32 = 2;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum SymAlg {
    Aes,
    Des3,
    Des3_112,
    Des,
    Rc2,
    Rc4,
}

impl SymAlg {
    pub(crate) fn block_size(self) -> usize {
        match self {
            Self::Aes => 16,
            Self::Des3 | Self::Des3_112 | Self::Des | Self::Rc2 => 8,
            Self::Rc4 => 0,
        }
    }

    fn as_name(self) -> &'static str {
        match self {
            Self::Aes => "AES",
            Self::Des3 => "3DES",
            Self::Des3_112 => "3DES_112",
            Self::Des => "DES",
            Self::Rc2 => "RC2",
            Self::Rc4 => "RC4",
        }
    }

    fn key_lengths(self) -> (u32, u32, u32) {
        match self {
            Self::Aes => (128, 256, 64),
            Self::Des3 => (192, 192, 0),
            Self::Des3_112 => (128, 128, 0),
            Self::Des => (64, 64, 0),
            Self::Rc2 => (40, 128, 8),
            Self::Rc4 => (8, 2048, 8),
        }
    }

    fn valid_key_len(self, len: usize) -> bool {
        match self {
            Self::Aes => matches!(len, 16 | 24 | 32),
            Self::Des3 => len == 24,
            Self::Des3_112 => len == 16,
            Self::Des => len == 8,
            Self::Rc2 => (1..=128).contains(&len),
            Self::Rc4 => (1..=256).contains(&len),
        }
    }

    fn default_mode(self) -> CipherMode {
        match self {
            Self::Rc4 => CipherMode::Na,
            _ => CipherMode::Cbc,
        }
    }

    fn supports_mode(self, mode: CipherMode) -> bool {
        match self {
            Self::Aes => matches!(
                mode,
                CipherMode::Ecb | CipherMode::Cbc | CipherMode::Cfb | CipherMode::Ctr | CipherMode::Gcm | CipherMode::Ccm
            ),
            Self::Des3 | Self::Des3_112 | Self::Des | Self::Rc2 => {
                matches!(mode, CipherMode::Ecb | CipherMode::Cbc | CipherMode::Cfb)
            }
            Self::Rc4 => mode == CipherMode::Na,
        }
    }

    fn to_u32(self) -> u32 {
        match self {
            Self::Aes => 0,
            Self::Des3 => 1,
            Self::Des3_112 => 2,
            Self::Des => 3,
            Self::Rc2 => 4,
            Self::Rc4 => 5,
        }
    }

    fn from_u32(v: u32) -> Option<Self> {
        match v {
            0 => Some(Self::Aes),
            1 => Some(Self::Des3),
            2 => Some(Self::Des3_112),
            3 => Some(Self::Des),
            4 => Some(Self::Rc2),
            5 => Some(Self::Rc4),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum CipherMode {
    Ecb,
    Cbc,
    Cfb,
    Ctr,
    Gcm,
    Ccm,
    Na,
}

impl CipherMode {
    fn from_name(s: &str) -> Option<Self> {
        let clean = s.trim_end_matches('\0');
        match clean {
            "ChainingModeECB" => Some(Self::Ecb),
            "ChainingModeCBC" => Some(Self::Cbc),
            "ChainingModeCFB" => Some(Self::Cfb),
            "ChainingModeCTR" => Some(Self::Ctr),
            "ChainingModeGCM" => Some(Self::Gcm),
            "ChainingModeCCM" => Some(Self::Ccm),
            "ChainingModeN/A" => Some(Self::Na),
            _ => None,
        }
    }

    fn as_win_name(self) -> &'static str {
        match self {
            Self::Ecb => "ChainingModeECB",
            Self::Cbc => "ChainingModeCBC",
            Self::Cfb => "ChainingModeCFB",
            Self::Ctr => "ChainingModeCTR",
            Self::Gcm => "ChainingModeGCM",
            Self::Ccm => "ChainingModeCCM",
            Self::Na => "ChainingModeN/A",
        }
    }

    fn to_u32(self) -> u32 {
        match self {
            Self::Ecb => 0,
            Self::Cbc => 1,
            Self::Cfb => 2,
            Self::Ctr => 3,
            Self::Gcm => 4,
            Self::Ccm => 5,
            Self::Na => 6,
        }
    }

    fn from_u32(v: u32) -> Option<Self> {
        match v {
            0 => Some(Self::Ecb),
            1 => Some(Self::Cbc),
            2 => Some(Self::Cfb),
            3 => Some(Self::Ctr),
            4 => Some(Self::Gcm),
            5 => Some(Self::Ccm),
            6 => Some(Self::Na),
            _ => None,
        }
    }
}

// --- RC4 stream cipher ---

#[derive(Clone)]
pub(crate) struct Rc4State {
    s: [u8; 256],
    i: u8,
    j: u8,
}

impl Rc4State {
    pub(crate) fn new(key: &[u8]) -> Self {
        let mut s = [0u8; 256];
        for (i, slot) in s.iter_mut().enumerate() {
            *slot = i as u8;
        }
        let mut j = 0u8;
        if !key.is_empty() {
            for i in 0..256 {
                j = j.wrapping_add(s[i]).wrapping_add(key[i % key.len()]);
                s.swap(i, j as usize);
            }
        }
        Self { s, i: 0, j: 0 }
    }

    pub(crate) fn apply(&mut self, data: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(data.len());
        for &b in data {
            self.i = self.i.wrapping_add(1);
            self.j = self.j.wrapping_add(self.s[self.i as usize]);
            self.s.swap(self.i as usize, self.j as usize);
            let k = self.s[self.s[self.i as usize].wrapping_add(self.s[self.j as usize]) as usize];
            out.push(b ^ k);
        }
        out
    }
}

// --- RFC 2268 RC2 64-bit block cipher (supports arbitrary effective key lengths) ---

const RC2_PITABLE: [u8; 256] = [
    0xd9, 0x78, 0xf9, 0xc4, 0x19, 0xdd, 0xb5, 0xed, 0x28, 0xe9, 0xfd, 0x79, 0x4a, 0xa0, 0xd8, 0x9d,
    0xc6, 0x7e, 0x37, 0x83, 0x2b, 0x76, 0x53, 0x8e, 0x62, 0x4c, 0x64, 0x88, 0x44, 0x8b, 0xfb, 0xa2,
    0x17, 0x9a, 0x59, 0xf5, 0x87, 0xb3, 0x4f, 0x13, 0x61, 0x45, 0x6d, 0x8d, 0x09, 0x81, 0x7d, 0x32,
    0xbd, 0x8f, 0x40, 0xeb, 0x86, 0xb7, 0x7b, 0x0b, 0xf0, 0x95, 0x21, 0x22, 0x5c, 0x6b, 0x4e, 0x82,
    0x54, 0xd6, 0x65, 0x93, 0xce, 0x60, 0xb2, 0x1c, 0x73, 0x56, 0xc0, 0x14, 0xa7, 0x8c, 0xf1, 0xdc,
    0x12, 0x75, 0xca, 0x1f, 0x3b, 0xbe, 0xe4, 0xd1, 0x42, 0x3d, 0xd4, 0x30, 0xa3, 0x3c, 0xb6, 0x26,
    0x6f, 0xbf, 0x0e, 0xda, 0x46, 0x69, 0x07, 0x57, 0x27, 0xf2, 0x1d, 0x9b, 0xbc, 0x94, 0x43, 0x03,
    0xf8, 0x11, 0xc7, 0xf6, 0x90, 0xef, 0x3e, 0xe7, 0x06, 0xc3, 0xd5, 0x2f, 0xc8, 0x66, 0x1e, 0xd7,
    0x08, 0xe8, 0xea, 0xde, 0x80, 0x52, 0xee, 0xf7, 0x84, 0xaa, 0x72, 0xac, 0x35, 0x4d, 0x6a, 0x2a,
    0x96, 0x1a, 0xd2, 0x71, 0x5a, 0x15, 0x49, 0x74, 0x4b, 0x9f, 0xd0, 0x5e, 0x04, 0x18, 0xa4, 0xec,
    0xc2, 0xe0, 0x41, 0x6e, 0x0f, 0x51, 0xcb, 0xcc, 0x24, 0x91, 0xaf, 0x50, 0xa1, 0xf4, 0x70, 0x39,
    0x99, 0x7c, 0x3a, 0x85, 0x23, 0xb8, 0xb4, 0x7a, 0xfc, 0x02, 0x36, 0x5b, 0x25, 0x55, 0x97, 0x31,
    0x2d, 0x5d, 0xfa, 0x98, 0xe3, 0x8a, 0x92, 0xae, 0x05, 0xdf, 0x29, 0x10, 0x67, 0x6c, 0xba, 0xc9,
    0xd3, 0x00, 0xe6, 0xcf, 0xe1, 0x9e, 0xa8, 0x2c, 0x63, 0x16, 0x01, 0x3f, 0x58, 0xe2, 0x89, 0xa9,
    0x0d, 0x38, 0x34, 0x1b, 0xab, 0x33, 0xff, 0xb0, 0xbb, 0x48, 0x0c, 0x5f, 0xb9, 0xb1, 0xcd, 0x2e,
    0xc5, 0xf3, 0xdb, 0x47, 0xe5, 0xa5, 0x9c, 0x77, 0x0a, 0xa6, 0x20, 0x68, 0xfe, 0x7f, 0xc1, 0xad,
];

fn rc2_expand_key(key: &[u8], effective_bits: u32) -> [u16; 64] {
    let t = key.len().clamp(1, 128);
    let t1 = if effective_bits == 0 {
        (t * 8).min(1024)
    } else {
        (effective_bits as usize).clamp(1, 1024)
    };
    let t8 = t1.div_ceil(8);
    let tm = 0xFFu8 >> (8 * t8 - t1);
    let mut l = [0u8; 128];
    l[..t].copy_from_slice(&key[..t]);
    for i in t..128 {
        l[i] = RC2_PITABLE[l[i - 1].wrapping_add(l[i - t]) as usize];
    }
    l[128 - t8] = RC2_PITABLE[(l[128 - t8] & tm) as usize];
    for i in (0..128 - t8).rev() {
        l[i] = RC2_PITABLE[(l[i + 1] ^ l[i + t8]) as usize];
    }
    let mut k = [0u16; 64];
    for i in 0..64 {
        k[i] = u16::from_le_bytes([l[2 * i], l[2 * i + 1]]);
    }
    k
}

fn rc2_encrypt_block(k: &[u16; 64], block: &[u8; 8]) -> [u8; 8] {
    let mut r = [
        u16::from_le_bytes([block[0], block[1]]),
        u16::from_le_bytes([block[2], block[3]]),
        u16::from_le_bytes([block[4], block[5]]),
        u16::from_le_bytes([block[6], block[7]]),
    ];
    let mut j = 0usize;
    let mix = |r: &mut [u16; 4], j: &mut usize| {
        r[0] = r[0].wrapping_add(k[*j]).wrapping_add(r[3] & r[2]).wrapping_add((!r[3]) & r[1]).rotate_left(1);
        *j += 1;
        r[1] = r[1].wrapping_add(k[*j]).wrapping_add(r[0] & r[3]).wrapping_add((!r[0]) & r[2]).rotate_left(2);
        *j += 1;
        r[2] = r[2].wrapping_add(k[*j]).wrapping_add(r[1] & r[0]).wrapping_add((!r[1]) & r[3]).rotate_left(3);
        *j += 1;
        r[3] = r[3].wrapping_add(k[*j]).wrapping_add(r[2] & r[1]).wrapping_add((!r[2]) & r[0]).rotate_left(5);
        *j += 1;
    };
    let mash = |r: &mut [u16; 4]| {
        r[0] = r[0].wrapping_add(k[(r[3] & 63) as usize]);
        r[1] = r[1].wrapping_add(k[(r[0] & 63) as usize]);
        r[2] = r[2].wrapping_add(k[(r[1] & 63) as usize]);
        r[3] = r[3].wrapping_add(k[(r[2] & 63) as usize]);
    };
    for _ in 0..5 {
        mix(&mut r, &mut j);
    }
    mash(&mut r);
    for _ in 0..6 {
        mix(&mut r, &mut j);
    }
    mash(&mut r);
    for _ in 0..5 {
        mix(&mut r, &mut j);
    }
    let mut out = [0u8; 8];
    for i in 0..4 {
        out[2 * i..2 * i + 2].copy_from_slice(&r[i].to_le_bytes());
    }
    out
}

fn rc2_decrypt_block(k: &[u16; 64], block: &[u8; 8]) -> [u8; 8] {
    let mut r = [
        u16::from_le_bytes([block[0], block[1]]),
        u16::from_le_bytes([block[2], block[3]]),
        u16::from_le_bytes([block[4], block[5]]),
        u16::from_le_bytes([block[6], block[7]]),
    ];
    let mut j = 63usize;
    let rmix = |r: &mut [u16; 4], j: &mut usize| {
        r[3] = r[3].rotate_right(5).wrapping_sub(k[*j]).wrapping_sub(r[2] & r[1]).wrapping_sub((!r[2]) & r[0]);
        *j = j.wrapping_sub(1);
        r[2] = r[2].rotate_right(3).wrapping_sub(k[*j]).wrapping_sub(r[1] & r[0]).wrapping_sub((!r[1]) & r[3]);
        *j = j.wrapping_sub(1);
        r[1] = r[1].rotate_right(2).wrapping_sub(k[*j]).wrapping_sub(r[0] & r[3]).wrapping_sub((!r[0]) & r[2]);
        *j = j.wrapping_sub(1);
        r[0] = r[0].rotate_right(1).wrapping_sub(k[*j]).wrapping_sub(r[3] & r[2]).wrapping_sub((!r[3]) & r[1]);
        *j = j.wrapping_sub(1);
    };
    let rmash = |r: &mut [u16; 4]| {
        r[3] = r[3].wrapping_sub(k[(r[2] & 63) as usize]);
        r[2] = r[2].wrapping_sub(k[(r[1] & 63) as usize]);
        r[1] = r[1].wrapping_sub(k[(r[0] & 63) as usize]);
        r[0] = r[0].wrapping_sub(k[(r[3] & 63) as usize]);
    };
    for _ in 0..5 {
        rmix(&mut r, &mut j);
    }
    rmash(&mut r);
    for _ in 0..6 {
        rmix(&mut r, &mut j);
    }
    rmash(&mut r);
    for _ in 0..5 {
        rmix(&mut r, &mut j);
    }
    let mut out = [0u8; 8];
    for i in 0..4 {
        out[2 * i..2 * i + 2].copy_from_slice(&r[i].to_le_bytes());
    }
    out
}

fn expand_des3_key(alg: SymAlg, key: &[u8]) -> Result<Vec<u8>, i32> {
    match alg {
        SymAlg::Des3 if key.len() == 24 => Ok(key.to_vec()),
        SymAlg::Des3_112 if key.len() == 16 => {
            let mut k = Vec::with_capacity(24);
            k.extend_from_slice(key);
            k.extend_from_slice(&key[..8]);
            Ok(k)
        }
        SymAlg::Des if key.len() == 8 => Ok(key.to_vec()),
        _ => Err(STATUS_INVALID_PARAMETER),
    }
}

fn cc_block_encrypt_single(cc_alg: u32, key: &[u8], block: &[u8]) -> Result<Vec<u8>, i32> {
    let mut out = vec![0u8; block.len()];
    let mut moved: usize = 0;
    let rc = unsafe {
        CCCrypt(
            KCC_ENCRYPT,
            cc_alg,
            KCC_OPTION_ECB_MODE,
            key.as_ptr() as *const c_void,
            key.len(),
            std::ptr::null(),
            block.as_ptr() as *const c_void,
            block.len(),
            out.as_mut_ptr() as *mut c_void,
            out.len(),
            &mut moved,
        )
    };
    if rc == 0 && moved == block.len() {
        Ok(out)
    } else {
        Err(STATUS_INVALID_PARAMETER)
    }
}

fn aes_encrypt_block(key: &[u8], block: &[u8; 16]) -> Result<[u8; 16], i32> {
    let mut out = [0u8; 16];
    let mut moved: usize = 0;
    let rc = unsafe {
        CCCrypt(
            KCC_ENCRYPT,
            KCC_ALGORITHM_AES,
            KCC_OPTION_ECB_MODE,
            key.as_ptr() as *const c_void,
            key.len(),
            std::ptr::null(),
            block.as_ptr() as *const c_void,
            16,
            out.as_mut_ptr() as *mut c_void,
            16,
            &mut moved,
        )
    };
    if rc == 0 && moved == 16 {
        Ok(out)
    } else {
        Err(STATUS_INVALID_PARAMETER)
    }
}

fn aes_decrypt_block(key: &[u8], block: &[u8; 16]) -> Result<[u8; 16], i32> {
    let mut out = [0u8; 16];
    let mut moved: usize = 0;
    let rc = unsafe {
        CCCrypt(
            KCC_DECRYPT,
            KCC_ALGORITHM_AES,
            KCC_OPTION_ECB_MODE,
            key.as_ptr() as *const c_void,
            key.len(),
            std::ptr::null(),
            block.as_ptr() as *const c_void,
            16,
            out.as_mut_ptr() as *mut c_void,
            16,
            &mut moved,
        )
    };
    if rc == 0 && moved == 16 {
        Ok(out)
    } else {
        Err(STATUS_INVALID_PARAMETER)
    }
}

fn pkcs7_pad(data: &[u8], bs: usize) -> Vec<u8> {
    let pad = bs - (data.len() % bs);
    let mut out = Vec::with_capacity(data.len() + pad);
    out.extend_from_slice(data);
    out.resize(data.len() + pad, pad as u8);
    out
}

fn pkcs7_unpad(data: &[u8], bs: usize) -> Result<Vec<u8>, i32> {
    if data.is_empty() || data.len() % bs != 0 {
        return Err(STATUS_INVALID_BUFFER_SIZE);
    }
    let pad = *data.last().unwrap() as usize;
    if pad == 0 || pad > bs || pad > data.len() {
        return Err(STATUS_DATA_ERROR);
    }
    if data[data.len() - pad..].iter().any(|&b| b as usize != pad) {
        return Err(STATUS_DATA_ERROR);
    }
    Ok(data[..data.len() - pad].to_vec())
}

fn cc_block_crypt(
    cc_alg: u32,
    bs: usize,
    encrypt: bool,
    ecb: bool,
    pkcs7: bool,
    key: &[u8],
    iv: Option<&[u8]>,
    input: &[u8],
) -> Result<Vec<u8>, i32> {
    if !pkcs7 && input.len() % bs != 0 {
        return Err(STATUS_INVALID_BUFFER_SIZE);
    }
    if !encrypt && pkcs7 && (input.is_empty() || input.len() % bs != 0) {
        return Err(STATUS_INVALID_BUFFER_SIZE);
    }
    if input.is_empty() && !pkcs7 {
        return Ok(Vec::new());
    }
    let op = if encrypt { KCC_ENCRYPT } else { KCC_DECRYPT };
    let mut opts = 0u32;
    if ecb {
        opts |= KCC_OPTION_ECB_MODE;
    }
    if pkcs7 {
        opts |= KCC_OPTION_PKCS7_PADDING;
    }
    let mut out = vec![0u8; input.len() + bs];
    let mut moved: usize = 0;
    let iv_ptr = iv.map_or(std::ptr::null(), |v| v.as_ptr() as *const c_void);
    let rc = unsafe {
        CCCrypt(
            op,
            cc_alg,
            opts,
            key.as_ptr() as *const c_void,
            key.len(),
            iv_ptr,
            input.as_ptr() as *const c_void,
            input.len(),
            out.as_mut_ptr() as *mut c_void,
            out.len(),
            &mut moved,
        )
    };
    if rc != 0 {
        return Err(if !encrypt && pkcs7 { STATUS_DATA_ERROR } else { STATUS_INVALID_PARAMETER });
    }
    out.truncate(moved);
    Ok(out)
}

fn rc2_crypt(
    encrypt: bool,
    mode: CipherMode,
    pkcs7: bool,
    key: &[u8],
    effective_bits: u32,
    iv: &mut [u8],
    msg_block_len: usize,
    input: &[u8],
) -> Result<Vec<u8>, i32> {
    let ks = rc2_expand_key(key, effective_bits);
    match mode {
        CipherMode::Ecb => {
            if !encrypt && input.len() % 8 != 0 {
                return Err(STATUS_INVALID_BUFFER_SIZE);
            }
            if encrypt && !pkcs7 && input.len() % 8 != 0 {
                return Err(STATUS_INVALID_BUFFER_SIZE);
            }
            let src = if encrypt && pkcs7 { pkcs7_pad(input, 8) } else { input.to_vec() };
            let mut out = Vec::with_capacity(src.len());
            for chunk in src.chunks_exact(8) {
                let blk: [u8; 8] = chunk.try_into().unwrap();
                let r = if encrypt { rc2_encrypt_block(&ks, &blk) } else { rc2_decrypt_block(&ks, &blk) };
                out.extend_from_slice(&r);
            }
            if !encrypt && pkcs7 {
                pkcs7_unpad(&out, 8)
            } else {
                Ok(out)
            }
        }
        CipherMode::Cbc => {
            if iv.len() < 8 {
                return Err(STATUS_INVALID_PARAMETER);
            }
            if !encrypt && input.len() % 8 != 0 {
                return Err(STATUS_INVALID_BUFFER_SIZE);
            }
            if encrypt && !pkcs7 && input.len() % 8 != 0 {
                return Err(STATUS_INVALID_BUFFER_SIZE);
            }
            let src = if encrypt && pkcs7 { pkcs7_pad(input, 8) } else { input.to_vec() };
            let mut chain: [u8; 8] = iv[..8].try_into().unwrap();
            let mut out = Vec::with_capacity(src.len());
            for chunk in src.chunks_exact(8) {
                let mut blk: [u8; 8] = chunk.try_into().unwrap();
                if encrypt {
                    for i in 0..8 {
                        blk[i] ^= chain[i];
                    }
                    let ct = rc2_encrypt_block(&ks, &blk);
                    chain = ct;
                    out.extend_from_slice(&ct);
                } else {
                    let mut pt = rc2_decrypt_block(&ks, &blk);
                    for i in 0..8 {
                        pt[i] ^= chain[i];
                    }
                    chain = blk;
                    out.extend_from_slice(&pt);
                }
            }
            iv[..8].copy_from_slice(&chain);
            if !encrypt && pkcs7 {
                pkcs7_unpad(&out, 8)
            } else {
                Ok(out)
            }
        }
        CipherMode::Cfb => {
            if iv.len() < 8 {
                return Err(STATUS_INVALID_PARAMETER);
            }
            let step = msg_block_len.clamp(1, 8);
            let mut chain: [u8; 8] = iv[..8].try_into().unwrap();
            let mut out = Vec::with_capacity(input.len());
            for chunk in input.chunks(step) {
                let enc = rc2_encrypt_block(&ks, &chain);
                let n = chunk.len();
                let mut ct = [0u8; 8];
                for i in 0..n {
                    let y = chunk[i] ^ enc[i];
                    out.push(y);
                    ct[i] = if encrypt { y } else { chunk[i] };
                }
                if n == 8 {
                    chain = ct;
                } else {
                    chain.copy_within(n..8, 0);
                    chain[8 - n..8].copy_from_slice(&ct[..n]);
                }
            }
            iv[..8].copy_from_slice(&chain);
            Ok(out)
        }
        _ => Err(STATUS_NOT_SUPPORTED),
    }
}

/// Shared block-cipher driver for AES, 3DES, 3DES_112, DES, and RC2 across ECB/CBC/CFB/CTR.
/// Updates `iv` in-place on CBC/CFB/CTR.
#[allow(clippy::too_many_arguments)]
pub(crate) fn sym_crypt_block_mode(
    alg: SymAlg,
    mode: CipherMode,
    encrypt: bool,
    pkcs7: bool,
    key: &[u8],
    effective_key_len: u32,
    iv: &mut [u8],
    msg_block_len: usize,
    input: &[u8],
) -> Result<Vec<u8>, i32> {
    if alg == SymAlg::Rc2 {
        return rc2_crypt(encrypt, mode, pkcs7, key, effective_key_len, iv, msg_block_len, input);
    }
    let bs = alg.block_size();
    let (cc_alg, cc_key_owned): (u32, Vec<u8>) = match alg {
        SymAlg::Aes => (KCC_ALGORITHM_AES, key.to_vec()),
        SymAlg::Des => (KCC_ALGORITHM_DES, expand_des3_key(alg, key)?),
        SymAlg::Des3 | SymAlg::Des3_112 => (KCC_ALGORITHM_3DES, expand_des3_key(alg, key)?),
        _ => return Err(STATUS_NOT_SUPPORTED),
    };
    match mode {
        CipherMode::Ecb => cc_block_crypt(cc_alg, bs, encrypt, true, pkcs7, &cc_key_owned, None, input),
        CipherMode::Cbc => {
            if iv.len() < bs {
                return Err(STATUS_INVALID_PARAMETER);
            }
            let next_iv_dec = if !encrypt && input.len() >= bs {
                Some(input[input.len() - bs..].to_vec())
            } else {
                None
            };
            let v = cc_block_crypt(cc_alg, bs, encrypt, false, pkcs7, &cc_key_owned, Some(&iv[..bs]), input)?;
            if encrypt && v.len() >= bs {
                iv[..bs].copy_from_slice(&v[v.len() - bs..]);
            } else if let Some(b) = next_iv_dec {
                iv[..bs].copy_from_slice(&b);
            }
            Ok(v)
        }
        CipherMode::Ctr if alg == SymAlg::Aes => {
            if iv.len() < 16 {
                return Err(STATUS_INVALID_PARAMETER);
            }
            let mut iv_blk: [u8; 16] = iv[..16].try_into().unwrap();
            let v = aes_ctr_crypt(&cc_key_owned, &mut iv_blk, input)?;
            iv[..16].copy_from_slice(&iv_blk);
            Ok(v)
        }
        CipherMode::Cfb => {
            if iv.len() < bs {
                return Err(STATUS_INVALID_PARAMETER);
            }
            let step = msg_block_len.clamp(1, bs);
            let mut chain = iv[..bs].to_vec();
            let mut out = Vec::with_capacity(input.len());
            for chunk in input.chunks(step) {
                let ks = cc_block_encrypt_single(cc_alg, &cc_key_owned, &chain)?;
                let n = chunk.len();
                let mut ct = vec![0u8; bs];
                for i in 0..n {
                    let y = chunk[i] ^ ks[i];
                    out.push(y);
                    ct[i] = if encrypt { y } else { chunk[i] };
                }
                if n == bs {
                    chain.copy_from_slice(&ct);
                } else {
                    chain.copy_within(n..bs, 0);
                    chain[bs - n..bs].copy_from_slice(&ct[..n]);
                }
            }
            iv[..bs].copy_from_slice(&chain);
            Ok(out)
        }
        _ => Err(STATUS_NOT_SUPPORTED),
    }
}

fn aes_ctr_crypt(key: &[u8], iv: &mut [u8; 16], input: &[u8]) -> Result<Vec<u8>, i32> {
    let mut out = Vec::with_capacity(input.len());
    for chunk in input.chunks(16) {
        let ks = aes_encrypt_block(key, iv)?;
        for (b, k) in chunk.iter().zip(ks.iter()) {
            out.push(b ^ k);
        }
        let c = u128::from_be_bytes(*iv).wrapping_add(1);
        *iv = c.to_be_bytes();
    }
    Ok(out)
}

/// RFC 4493 AES-CMAC (16-byte MAC for AES-128/192/256 keys).
fn aes_cmac(key: &[u8], data: &[u8]) -> Result<[u8; 16], i32> {
    if !matches!(key.len(), 16 | 24 | 32) {
        return Err(STATUS_INVALID_PARAMETER);
    }
    let dbl = |b: [u8; 16]| -> [u8; 16] {
        let mut v = u128::from_be_bytes(b);
        let msb = (v >> 127) != 0;
        v <<= 1;
        if msb {
            v ^= 0x87;
        }
        v.to_be_bytes()
    };
    let l = aes_encrypt_block(key, &[0u8; 16])?;
    let k1 = dbl(l);
    let k2 = dbl(k1);

    let n = if data.is_empty() { 1 } else { data.len().div_ceil(16) };
    let complete = !data.is_empty() && data.len() % 16 == 0;
    let mut m_last = [0u8; 16];
    if complete {
        let tail = &data[(n - 1) * 16..];
        for i in 0..16 {
            m_last[i] = tail[i] ^ k1[i];
        }
    } else {
        let tail = if data.is_empty() { &[] } else { &data[(n - 1) * 16..] };
        m_last[..tail.len()].copy_from_slice(tail);
        m_last[tail.len()] = 0x80;
        for i in 0..16 {
            m_last[i] ^= k2[i];
        }
    }
    let mut x = [0u8; 16];
    for blk in 0..n - 1 {
        let chunk = &data[blk * 16..(blk + 1) * 16];
        let mut y = [0u8; 16];
        for i in 0..16 {
            y[i] = x[i] ^ chunk[i];
        }
        x = aes_encrypt_block(key, &y)?;
    }
    let mut y = [0u8; 16];
    for i in 0..16 {
        y[i] = x[i] ^ m_last[i];
    }
    aes_encrypt_block(key, &y)
}

/// RFC 3394 / RFC 3565 AES Key Wrap.
fn aes_key_wrap(kek: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, i32> {
    if !matches!(kek.len(), 16 | 24 | 32) || plaintext.len() < 16 || plaintext.len() % 8 != 0 {
        return Err(STATUS_INVALID_PARAMETER);
    }
    let n = plaintext.len() / 8;
    let mut a: u64 = 0xA6A6_A6A6_A6A6_A6A6;
    let mut r: Vec<[u8; 8]> = plaintext.chunks_exact(8).map(|c| c.try_into().unwrap()).collect();
    for j in 0..6usize {
        for i in 0..n {
            let mut blk = [0u8; 16];
            blk[0..8].copy_from_slice(&a.to_be_bytes());
            blk[8..16].copy_from_slice(&r[i]);
            let b = aes_encrypt_block(kek, &blk)?;
            let t = (n * j + i + 1) as u64;
            a = u64::from_be_bytes(b[0..8].try_into().unwrap()) ^ t;
            r[i].copy_from_slice(&b[8..16]);
        }
    }
    let mut out = Vec::with_capacity(8 + plaintext.len());
    out.extend_from_slice(&a.to_be_bytes());
    for seg in r {
        out.extend_from_slice(&seg);
    }
    Ok(out)
}

/// RFC 3394 / RFC 3565 AES Key Unwrap.
fn aes_key_unwrap(kek: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, i32> {
    if !matches!(kek.len(), 16 | 24 | 32) || ciphertext.len() < 24 || ciphertext.len() % 8 != 0 {
        return Err(STATUS_INVALID_PARAMETER);
    }
    let n = ciphertext.len() / 8 - 1;
    let mut a = u64::from_be_bytes(ciphertext[0..8].try_into().unwrap());
    let mut r: Vec<[u8; 8]> = ciphertext[8..].chunks_exact(8).map(|c| c.try_into().unwrap()).collect();
    for j in (0..6usize).rev() {
        for i in (0..n).rev() {
            let t = (n * j + i + 1) as u64;
            let mut blk = [0u8; 16];
            blk[0..8].copy_from_slice(&(a ^ t).to_be_bytes());
            blk[8..16].copy_from_slice(&r[i]);
            let b = aes_decrypt_block(kek, &blk)?;
            a = u64::from_be_bytes(b[0..8].try_into().unwrap());
            r[i].copy_from_slice(&b[8..16]);
        }
    }
    if a != 0xA6A6_A6A6_A6A6_A6A6 {
        return Err(STATUS_DATA_ERROR);
    }
    let mut out = Vec::with_capacity(n * 8);
    for seg in r {
        out.extend_from_slice(&seg);
    }
    Ok(out)
}

/// NIST SP 800-38C AES-CCM encrypt/decrypt.
fn aes_ccm_crypt(
    encrypt: bool,
    key: &[u8],
    nonce: &[u8],
    aad: &[u8],
    input: &[u8],
    tag_len: usize,
    expected_tag: Option<&[u8]>,
) -> Result<(Vec<u8>, Vec<u8>), i32> {
    if !(7..=13).contains(&nonce.len()) || !matches!(tag_len, 4 | 6 | 8 | 10 | 12 | 14 | 16) {
        return Err(STATUS_INVALID_PARAMETER);
    }
    let q = 15 - nonce.len(); // 2..=8
    let payload_len = input.len() as u64;
    if q < 8 && payload_len >= (1u64 << (8 * q)) {
        return Err(STATUS_INVALID_PARAMETER);
    }

    let make_ctr = |counter: u64| -> [u8; 16] {
        let mut blk = [0u8; 16];
        blk[0] = (q - 1) as u8;
        blk[1..1 + nonce.len()].copy_from_slice(nonce);
        let be = counter.to_be_bytes();
        blk[16 - q..16].copy_from_slice(&be[8 - q..8]);
        blk
    };

    let s0 = aes_encrypt_block(key, &make_ctr(0))?;
    let mut out = Vec::with_capacity(input.len());
    for (idx, chunk) in input.chunks(16).enumerate() {
        let si = aes_encrypt_block(key, &make_ctr((idx + 1) as u64))?;
        for (b, k) in chunk.iter().zip(si.iter()) {
            out.push(b ^ k);
        }
    }

    let plaintext: &[u8] = if encrypt { input } else { &out };

    // CBC-MAC: B_0 || formatted AAD || zero-padded plaintext
    let mut b0 = [0u8; 16];
    let has_aad = if aad.is_empty() { 0u8 } else { 0x40u8 };
    b0[0] = has_aad | ((((tag_len - 2) / 2) as u8) << 3) | ((q - 1) as u8);
    b0[1..1 + nonce.len()].copy_from_slice(nonce);
    let len_be = payload_len.to_be_bytes();
    b0[16 - q..16].copy_from_slice(&len_be[8 - q..8]);

    let mut y = aes_encrypt_block(key, &b0)?;
    let mut cbc_feed = |buf: &[u8]| -> Result<(), i32> {
        for chunk in buf.chunks(16) {
            let mut blk = y;
            for (i, &b) in chunk.iter().enumerate() {
                blk[i] ^= b;
            }
            y = aes_encrypt_block(key, &blk)?;
        }
        Ok(())
    };

    if !aad.is_empty() {
        let mut aad_fmt = Vec::with_capacity(aad.len() + 10);
        if aad.len() < 0xFF00 {
            aad_fmt.extend_from_slice(&(aad.len() as u16).to_be_bytes());
        } else if aad.len() <= u32::MAX as usize {
            aad_fmt.push(0xFF);
            aad_fmt.push(0xFE);
            aad_fmt.extend_from_slice(&(aad.len() as u32).to_be_bytes());
        } else {
            aad_fmt.push(0xFF);
            aad_fmt.push(0xFF);
            aad_fmt.extend_from_slice(&(aad.len() as u64).to_be_bytes());
        }
        aad_fmt.extend_from_slice(aad);
        cbc_feed(&aad_fmt)?;
    }
    if !plaintext.is_empty() {
        cbc_feed(plaintext)?;
    }

    let mut tag = vec![0u8; tag_len];
    for i in 0..tag_len {
        tag[i] = y[i] ^ s0[i];
    }

    if !encrypt {
        let Some(exp) = expected_tag else {
            return Err(STATUS_INVALID_PARAMETER);
        };
        if exp.len() != tag_len {
            return Err(STATUS_INVALID_PARAMETER);
        }
        let mut diff = 0u8;
        for (&a, &b) in tag.iter().zip(exp.iter()) {
            diff |= a ^ b;
        }
        if diff != 0 {
            return Err(STATUS_AUTH_TAG_MISMATCH);
        }
    }

    Ok((out, tag))
}

fn gf128_mul(x: u128, y: u128) -> u128 {
    const R: u128 = 0xE100_0000_0000_0000_0000_0000_0000_0000;
    let mut z: u128 = 0;
    let mut v: u128 = y;
    for i in 0..128 {
        if ((x >> (127 - i)) & 1) != 0 {
            z ^= v;
        }
        let lsb = v & 1;
        v >>= 1;
        if lsb != 0 {
            v ^= R;
        }
    }
    z
}

fn ghash_update(mut y: u128, h: u128, data: &[u8]) -> u128 {
    for chunk in data.chunks(16) {
        let mut blk = [0u8; 16];
        blk[..chunk.len()].copy_from_slice(chunk);
        y = gf128_mul(y ^ u128::from_be_bytes(blk), h);
    }
    y
}

fn gcm_inc32(ctr: &mut [u8; 16]) {
    let n = u32::from_be_bytes(ctr[12..16].try_into().unwrap()).wrapping_add(1);
    ctr[12..16].copy_from_slice(&n.to_be_bytes());
}

fn aes_gcm_crypt(
    encrypt: bool,
    key: &[u8],
    nonce: &[u8],
    aad: &[u8],
    input: &[u8],
    tag_len: usize,
    expected_tag: Option<&[u8]>,
) -> Result<(Vec<u8>, Vec<u8>), i32> {
    if nonce.is_empty() || !(12..=16).contains(&tag_len) {
        return Err(STATUS_INVALID_PARAMETER);
    }
    let h_bytes = aes_encrypt_block(key, &[0u8; 16])?;
    let h = u128::from_be_bytes(h_bytes);

    let j0: [u8; 16] = if nonce.len() == 12 {
        let mut b = [0u8; 16];
        b[..12].copy_from_slice(nonce);
        b[15] = 1;
        b
    } else {
        let mut y = ghash_update(0, h, nonce);
        let mut len_blk = [0u8; 16];
        len_blk[8..16].copy_from_slice(&((nonce.len() as u64) * 8).to_be_bytes());
        y = gf128_mul(y ^ u128::from_be_bytes(len_blk), h);
        y.to_be_bytes()
    };

    let mut ctr = j0;
    let mut out = Vec::with_capacity(input.len());
    for chunk in input.chunks(16) {
        gcm_inc32(&mut ctr);
        let ks = aes_encrypt_block(key, &ctr)?;
        for (b, k) in chunk.iter().zip(ks.iter()) {
            out.push(b ^ k);
        }
    }

    let ciphertext: &[u8] = if encrypt { &out } else { input };
    let mut s = ghash_update(0, h, aad);
    s = ghash_update(s, h, ciphertext);
    let mut lens = [0u8; 16];
    lens[0..8].copy_from_slice(&((aad.len() as u64) * 8).to_be_bytes());
    lens[8..16].copy_from_slice(&((ciphertext.len() as u64) * 8).to_be_bytes());
    s = gf128_mul(s ^ u128::from_be_bytes(lens), h);

    let ej0 = u128::from_be_bytes(aes_encrypt_block(key, &j0)?);
    let full_tag = (s ^ ej0).to_be_bytes();
    let tag = full_tag[..tag_len].to_vec();

    if !encrypt {
        let Some(exp) = expected_tag else {
            return Err(STATUS_INVALID_PARAMETER);
        };
        if exp.len() != tag_len {
            return Err(STATUS_INVALID_PARAMETER);
        }
        let mut diff = 0u8;
        for (&a, &b) in tag.iter().zip(exp.iter()) {
            diff |= a ^ b;
        }
        if diff != 0 {
            return Err(STATUS_AUTH_TAG_MISMATCH);
        }
    }

    Ok((out, tag))
}

#[repr(C)]
struct BcryptAuthCipherModeInfo {
    cb_size: u32,
    dw_info_version: u32,
    pb_nonce: *mut u8,
    cb_nonce: u32,
    pb_auth_data: *mut u8,
    cb_auth_data: u32,
    pb_tag: *mut u8,
    cb_tag: u32,
    pb_mac_context: *mut u8,
    cb_mac_context: u32,
    cb_aad: u32,
    cb_data: u64,
    dw_flags: u32,
}

// ---------------------------------------------------------------------------
// Handles
// ---------------------------------------------------------------------------

const ALG_MAGIC: u32 = 0x414C_4731; // "ALG1"
const HASH_MAGIC: u32 = 0x4853_4831; // "HSH1"
const KEY_MAGIC: u32 = 0x4B45_5931; // "KEY1"
const SECRET_MAGIC: u32 = 0x5345_4331; // "SEC1"

const KDF_HASH_ALGORITHM: u32 = 0x0;
const KDF_SECRET_PREPEND: u32 = 0x1;
const KDF_SECRET_APPEND: u32 = 0x2;
const KDF_HMAC_KEY: u32 = 0x3;
const KDF_TLS_PRF_LABEL: u32 = 0x4;
const KDF_TLS_PRF_SEED: u32 = 0x5;
const KDF_ALGORITHMID: u32 = 0x8;
const KDF_PARTYUINFO: u32 = 0x9;
const KDF_PARTYVINFO: u32 = 0xA;
const KDF_SUPPPUBINFO: u32 = 0xB;
const KDF_SUPPPRIVINFO: u32 = 0xC;
const KDF_USE_SECRET_AS_HMAC_KEY_FLAG: u32 = 0x1;

#[repr(C)]
struct BcryptOaepPaddingInfo {
    psz_alg_id: *const u16,
    pb_label: *const u8,
    cb_label: u32,
}

#[repr(C)]
struct BcryptBuffer {
    cb_buffer: u32,
    buffer_type: u32,
    pv_buffer: *mut c_void,
}

#[repr(C)]
struct BcryptBufferDesc {
    ul_version: u32,
    c_buffers: u32,
    p_buffers: *mut BcryptBuffer,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum AlgKind {
    Hash(Digest),
    AesCmac,
    AesGmac,
    Symmetric(SymAlg),
    Rsa,
    Ecc { usage: EccUsage, curve: Option<EccCurve> },
    Rng,
    Kdf,
}

impl AlgKind {
    fn primitive_type(self) -> u32 {
        match self {
            Self::Symmetric(_) => BCRYPT_CIPHER_INTERFACE,
            Self::Hash(_) | Self::AesCmac | Self::AesGmac => BCRYPT_HASH_INTERFACE,
            Self::Rsa => BCRYPT_ASYMMETRIC_ENCRYPTION_INTERFACE,
            Self::Ecc { usage: EccUsage::Ecdh, .. } => BCRYPT_SECRET_AGREEMENT_INTERFACE,
            Self::Ecc { usage: EccUsage::Ecdsa, .. } => BCRYPT_SIGNATURE_INTERFACE,
            Self::Rng => BCRYPT_RNG_INTERFACE,
            Self::Kdf => BCRYPT_KEY_DERIVATION_INTERFACE,
        }
    }
}

#[repr(C)]
struct Alg {
    magic: u32,
    kind: AlgKind,
    hmac: bool,
    reusable: bool,
    mode: CipherMode,
    msg_block_len: u32,
    effective_key_len: u32,
    ecc_curve: Option<EccCurve>,
    name: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum HashKind {
    Digest(Digest),
    AesCmac,
    AesGmac,
}

impl HashKind {
    fn len(self) -> usize {
        match self {
            Self::Digest(d) => d.len(),
            Self::AesCmac | Self::AesGmac => 16,
        }
    }
    fn block_len(self) -> u32 {
        match self {
            Self::Digest(d) => d.block_len(),
            Self::AesCmac | Self::AesGmac => 16,
        }
    }
    fn as_name(self) -> &'static str {
        match self {
            Self::Digest(d) => d.as_name(),
            Self::AesCmac => "AES-CMAC",
            Self::AesGmac => "AES-GMAC",
        }
    }
}

#[repr(C)]
struct Hash {
    magic: u32,
    kind: HashKind,
    key: Option<Vec<u8>>,
    iv: Vec<u8>,
    reusable: bool,
    data: Vec<u8>,
}

enum KeyData {
    Rsa(RsaKey),
    PendingRsa {
        bits: u32,
    },
    Ecc(EccKey),
    PendingEcc {
        usage: EccUsage,
        curve: Option<EccCurve>,
    },
    Symmetric {
        alg: SymAlg,
        key: Vec<u8>,
        mode: CipherMode,
        msg_block_len: u32,
        effective_key_len: u32,
        iv: Vec<u8>,
        rc4: Option<Rc4State>,
    },
}

#[repr(C)]
struct Key {
    magic: u32,
    data: KeyData,
}

#[repr(C)]
struct Secret {
    magic: u32,
    secret: Vec<u8>,
}

unsafe fn alg<'a>(h: *mut c_void) -> Option<&'a Alg> {
    let a = h as *const Alg;
    (!a.is_null() && (*a).magic == ALG_MAGIC).then(|| &*a)
}
unsafe fn alg_mut<'a>(h: *mut c_void) -> Option<&'a mut Alg> {
    let a = h as *mut Alg;
    (!a.is_null() && (*a).magic == ALG_MAGIC).then(|| &mut *a)
}
unsafe fn hash_obj<'a>(h: *mut c_void) -> Option<&'a mut Hash> {
    let a = h as *mut Hash;
    (!a.is_null() && (*a).magic == HASH_MAGIC).then(|| &mut *a)
}
unsafe fn key<'a>(h: *mut c_void) -> Option<&'a Key> {
    let a = h as *const Key;
    (!a.is_null() && (*a).magic == KEY_MAGIC).then(|| &*a)
}
unsafe fn key_mut<'a>(h: *mut c_void) -> Option<&'a mut Key> {
    let a = h as *mut Key;
    (!a.is_null() && (*a).magic == KEY_MAGIC).then(|| &mut *a)
}
unsafe fn secret_obj<'a>(h: *mut c_void) -> Option<&'a Secret> {
    let s = h as *const Secret;
    (!s.is_null() && (*s).magic == SECRET_MAGIC).then(|| &*s)
}

unsafe fn bytes<'a>(p: *const u8, n: u32) -> &'a [u8] {
    if p.is_null() || n == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(p, n as usize)
    }
}

// ---------------------------------------------------------------------------
// Exports
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn BCryptGenRandom(_alg: *mut c_void, buf: *mut u8, len: u32, _flags: u32) -> i32 {
    if buf.is_null() && len > 0 {
        return STATUS_INVALID_PARAMETER;
    }
    if !crate::det_entropy::fill(buf, len as usize) {
        libc::arc4random_buf(buf as *mut c_void, len as usize);
    }
    STATUS_SUCCESS
}

/// `bcryptprimitives.dll!ProcessPrng(PVOID pbData, SIZE_T cbData) -> BOOL`.
/// Uses the shared deterministic-entropy hook (when active in difftests) and
/// falls back to Darwin `arc4random_buf` (identical on macOS and iOS).
pub unsafe extern "win64" fn ProcessPrng(buf: *mut u8, len: usize) -> crate::types::BOOL {
    if len == 0 {
        return crate::types::TRUE;
    }
    if buf.is_null() {
        crate::teb::set_last_error(crate::types::err::ERROR_INVALID_PARAMETER);
        return crate::types::FALSE;
    }
    if !crate::det_entropy::fill(buf, len) {
        libc::arc4random_buf(buf as *mut c_void, len);
    }
    crate::types::TRUE
}

pub unsafe extern "win64" fn BCryptOpenAlgorithmProvider(out: *mut *mut c_void, alg_id: *const u16, _impl: *const u16, flags: u32) -> i32 {
    if out.is_null() || alg_id.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    if (flags & !(BCRYPT_ALG_HANDLE_HMAC_FLAG | BCRYPT_HASH_REUSABLE_FLAG)) != 0 {
        return STATUS_INVALID_PARAMETER;
    }
    let name = crate::strings::from_wide(alg_id);
    let hmac = flags & BCRYPT_ALG_HANDLE_HMAC_FLAG != 0;
    let reusable = flags & BCRYPT_HASH_REUSABLE_FLAG != 0;
    let upper = name.to_ascii_uppercase();
    let kind = if let Some(d) = Digest::from_name(&name) {
        AlgKind::Hash(d)
    } else if upper == "AES-CMAC" {
        AlgKind::AesCmac
    } else if upper == "AES-GMAC" {
        AlgKind::AesGmac
    } else if !hmac && !reusable {
        match upper.as_str() {
            "AES" => AlgKind::Symmetric(SymAlg::Aes),
            "3DES" => AlgKind::Symmetric(SymAlg::Des3),
            "3DES_112" => AlgKind::Symmetric(SymAlg::Des3_112),
            "DES" => AlgKind::Symmetric(SymAlg::Des),
            "RC2" => AlgKind::Symmetric(SymAlg::Rc2),
            "RC4" => AlgKind::Symmetric(SymAlg::Rc4),
            "RSA" | "RSA_SIGN" => AlgKind::Rsa,
            "ECDSA_P256" => AlgKind::Ecc { usage: EccUsage::Ecdsa, curve: Some(EccCurve::P256) },
            "ECDSA_P384" => AlgKind::Ecc { usage: EccUsage::Ecdsa, curve: Some(EccCurve::P384) },
            "ECDSA_P521" => AlgKind::Ecc { usage: EccUsage::Ecdsa, curve: Some(EccCurve::P521) },
            "ECDSA" => AlgKind::Ecc { usage: EccUsage::Ecdsa, curve: None },
            "ECDH_P256" => AlgKind::Ecc { usage: EccUsage::Ecdh, curve: Some(EccCurve::P256) },
            "ECDH_P384" => AlgKind::Ecc { usage: EccUsage::Ecdh, curve: Some(EccCurve::P384) },
            "ECDH_P521" => AlgKind::Ecc { usage: EccUsage::Ecdh, curve: Some(EccCurve::P521) },
            "ECDH" => AlgKind::Ecc { usage: EccUsage::Ecdh, curve: None },
            "RNG" | "FIPS186DSARNG" => AlgKind::Rng,
            "PBKDF2" | "SP800_108_CTR_HMAC" | "SP800_56A_CONCAT" | "CAPI_KDF" | "TLS1_1_KDF" | "TLS1_2_KDF" | "HKDF" => {
                AlgKind::Kdf
            }
            _ => {
                // Unknown algorithm identifier: Windows CNG returns STATUS_NOT_FOUND.
                log::debug!("BCryptOpenAlgorithmProvider({name:?}, flags 0x{flags:x}) -> STATUS_NOT_FOUND");
                return STATUS_NOT_FOUND;
            }
        }
    } else {
        // HMAC/reusable flags on a non-hash algorithm: Windows returns STATUS_NOT_FOUND / STATUS_INVALID_PARAMETER.
        log::debug!("BCryptOpenAlgorithmProvider({name:?}, flags 0x{flags:x}) -> STATUS_NOT_FOUND");
        return STATUS_NOT_FOUND;
    };
    let (mode, msg_block_len) = match kind {
        AlgKind::Symmetric(s) => (s.default_mode(), s.block_size() as u32),
        _ => (CipherMode::Cbc, 16),
    };
    let ecc_curve = match kind {
        AlgKind::Ecc { curve, .. } => curve,
        _ => None,
    };
    log::debug!("BCryptOpenAlgorithmProvider({name:?}, flags 0x{flags:x}) -> {kind:?}");
    std::ptr::write_unaligned(
        out,
        Box::into_raw(Box::new(Alg {
            magic: ALG_MAGIC,
            kind,
            hmac,
            reusable,
            mode,
            msg_block_len,
            effective_key_len: 0,
            ecc_curve,
            name,
        })) as *mut c_void,
    );
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn BCryptCloseAlgorithmProvider(h: *mut c_void, _flags: u32) -> i32 {
    if alg(h).is_none() {
        return STATUS_INVALID_HANDLE;
    }
    let mut a = Box::from_raw(h as *mut Alg);
    a.magic = 0;
    STATUS_SUCCESS
}

unsafe fn put_property(value: &[u8], out: *mut u8, out_len: u32, result: *mut u32) -> i32 {
    if result.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    std::ptr::write_unaligned(result, value.len() as u32);
    if out.is_null() {
        return STATUS_SUCCESS;
    }
    if (out_len as usize) < value.len() {
        return STATUS_BUFFER_TOO_SMALL;
    }
    std::ptr::copy_nonoverlapping(value.as_ptr(), out, value.len());
    STATUS_SUCCESS
}

unsafe fn put_ecc_curve_name_list(out: *mut u8, out_len: u32, result: *mut u32) -> i32 {
    if result.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    const CURVES: &[&str] = &["nistP256", "nistP384", "nistP521"];
    // BCRYPT_ECC_CURVE_NAMES (16 bytes on x64) + 3 * 8-byte LPWSTR array + UTF-16LE strings
    let header_sz = 16usize;
    let ptrs_sz = CURVES.len() * 8;
    let wide_strs: Vec<Vec<u8>> = CURVES.iter().map(|s| wide_bytes(s)).collect();
    let strs_sz: usize = wide_strs.iter().map(Vec::len).sum();
    let total = header_sz + ptrs_sz + strs_sz;
    std::ptr::write_unaligned(result, total as u32);
    if out.is_null() {
        return STATUS_SUCCESS;
    }
    if (out_len as usize) < total {
        return STATUS_BUFFER_TOO_SMALL;
    }
    std::ptr::write_bytes(out, 0, total);
    std::ptr::write_unaligned(out as *mut u32, CURVES.len() as u32);
    let ptrs_base = out.add(header_sz);
    std::ptr::write_unaligned(out.add(8) as *mut usize, ptrs_base as usize);
    let mut str_off = header_sz + ptrs_sz;
    for (i, ws) in wide_strs.iter().enumerate() {
        let dst = out.add(str_off);
        std::ptr::copy_nonoverlapping(ws.as_ptr(), dst, ws.len());
        std::ptr::write_unaligned(ptrs_base.add(i * 8) as *mut usize, dst as usize);
        str_off += ws.len();
    }
    STATUS_SUCCESS
}

fn wide_bytes(s: &str) -> Vec<u8> {
    let mut w: Vec<u8> = s.encode_utf16().flat_map(u16::to_le_bytes).collect();
    w.extend([0, 0]);
    w
}

fn auth_tag_lengths_bytes(mode: CipherMode) -> Option<Vec<u8>> {
    match mode {
        // BCRYPT_AUTH_TAG_LENGTHS_STRUCT { dwMinLength, dwMaxLength, dwIncrement }
        CipherMode::Gcm => Some([12u32, 16, 1].into_iter().flat_map(u32::to_le_bytes).collect()),
        CipherMode::Ccm => Some([4u32, 16, 2].into_iter().flat_map(u32::to_le_bytes).collect()),
        // Default GCM tag lengths when queried before setting ChainingMode (preserves existing behavior)
        _ => Some([12u32, 16, 1].into_iter().flat_map(u32::to_le_bytes).collect()),
    }
}

pub unsafe extern "win64" fn BCryptGetProperty(h: *mut c_void, prop: *const u16, out: *mut u8, out_len: u32, result: *mut u32, _flags: u32) -> i32 {
    if prop.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let p = crate::strings::from_wide(prop);
    let u32v = |v: u32| v.to_le_bytes().to_vec();
    let key_lengths = |min: u32, max: u32, inc: u32| -> Vec<u8> {
        [min, max, inc].into_iter().flat_map(u32::to_le_bytes).collect()
    };

    if let Some(a) = alg(h) {
        if p == "ECCCurveNameList" && matches!(a.kind, AlgKind::Ecc { .. }) {
            return put_ecc_curve_name_list(out, out_len, result);
        }
        let value = match (p.as_str(), a.kind) {
            ("ObjectLength" | "MultiObjectLength", _) => u32v(OBJECT_LENGTH),
            ("AlgorithmName", _) => wide_bytes(&a.name),
            ("PrimitiveType", k) => u32v(k.primitive_type()),
            ("IsKeyedHash", AlgKind::Hash(_)) => u32v(a.hmac as u32),
            ("IsKeyedHash", AlgKind::AesCmac | AlgKind::AesGmac) => u32v(1),
            ("IsReusableHash", AlgKind::Hash(_) | AlgKind::AesCmac | AlgKind::AesGmac) => u32v(a.reusable as u32),
            ("HashDigestLength", AlgKind::Hash(d)) => u32v(d.len() as u32),
            ("HashBlockLength", AlgKind::Hash(d)) => u32v(d.block_len()),
            ("HashDigestLength", AlgKind::AesCmac | AlgKind::AesGmac) => u32v(16),
            ("HashBlockLength", AlgKind::AesCmac | AlgKind::AesGmac) => u32v(16),
            ("BlockLength" | "BlockSizeList", AlgKind::Symmetric(s)) => u32v(s.block_size() as u32),
            ("MessageBlockLength", AlgKind::Symmetric(s)) if s.block_size() > 0 => u32v(a.msg_block_len),
            ("ChainingMode", AlgKind::Symmetric(_)) => wide_bytes(a.mode.as_win_name()),
            ("KeyLengths", AlgKind::Symmetric(s)) => {
                let (min, max, inc) = s.key_lengths();
                key_lengths(min, max, inc)
            }
            ("EffectiveKeyLength", AlgKind::Symmetric(SymAlg::Rc2)) => {
                u32v(if a.effective_key_len == 0 { 128 } else { a.effective_key_len })
            }
            ("AuthTagLength", AlgKind::Symmetric(SymAlg::Aes)) => auth_tag_lengths_bytes(a.mode).unwrap(),
            ("KeyLengths", AlgKind::Rsa) => key_lengths(512, 16384, 64),
            ("PaddingSchemes", AlgKind::Rsa) => u32v(
                BCRYPT_SUPPORTED_PAD_PKCS1_ENC
                    | BCRYPT_SUPPORTED_PAD_PKCS1_SIG
                    | BCRYPT_SUPPORTED_PAD_OAEP
                    | BCRYPT_SUPPORTED_PAD_PSS,
            ),
            ("KeyLengths", AlgKind::Ecc { curve: Some(c), .. }) => key_lengths(c.bits(), c.bits(), 0),
            ("KeyLengths", AlgKind::Ecc { curve: None, .. }) => match a.ecc_curve {
                Some(c) => key_lengths(c.bits(), c.bits(), 0),
                None => key_lengths(256, 521, 0),
            },
            ("ECCCurveName", AlgKind::Ecc { .. }) => match a.ecc_curve {
                Some(c) => wide_bytes(c.win_curve_name()),
                None => return STATUS_NOT_FOUND,
            },
            _ => {
                // Property is not defined on this algorithm handle per CNG specification.
                log::debug!("BCryptGetProperty(alg={}, {p:?}) -> STATUS_NOT_SUPPORTED", a.name);
                return STATUS_NOT_SUPPORTED;
            }
        };
        return put_property(&value, out, out_len, result);
    }

    if let Some(o) = hash_obj(h) {
        let value = match p.as_str() {
            "ObjectLength" => u32v(OBJECT_LENGTH),
            "HashDigestLength" => u32v(o.kind.len() as u32),
            "HashBlockLength" => u32v(o.kind.block_len()),
            "AlgorithmName" => wide_bytes(o.kind.as_name()),
            "IsKeyedHash" => u32v(o.key.is_some() as u32),
            "IsReusableHash" => u32v(o.reusable as u32),
            "PrimitiveType" => u32v(BCRYPT_HASH_INTERFACE),
            _ => {
                log::debug!("BCryptGetProperty(hash, {p:?}) -> STATUS_NOT_SUPPORTED");
                return STATUS_NOT_SUPPORTED;
            }
        };
        return put_property(&value, out, out_len, result);
    }

    if let Some(k) = key(h) {
        let value = match (&k.data, p.as_str()) {
            (_, "ObjectLength") => u32v(OBJECT_LENGTH),
            (KeyData::Symmetric { alg, .. }, "BlockLength" | "BlockSizeList") => u32v(alg.block_size() as u32),
            (KeyData::Symmetric { key, .. }, "KeyLength" | "KeyStrength") => u32v((key.len() * 8) as u32),
            (KeyData::Symmetric { alg, .. }, "KeyLengths") => {
                let (min, max, inc) = alg.key_lengths();
                key_lengths(min, max, inc)
            }
            (KeyData::Symmetric { mode, .. }, "ChainingMode") => wide_bytes(mode.as_win_name()),
            (KeyData::Symmetric { msg_block_len, alg, .. }, "MessageBlockLength") if alg.block_size() > 0 => {
                u32v(*msg_block_len)
            }
            (KeyData::Symmetric { alg: SymAlg::Aes, mode, .. }, "AuthTagLength") => auth_tag_lengths_bytes(*mode).unwrap(),
            (KeyData::Symmetric { alg: SymAlg::Rc2, effective_key_len, key, .. }, "EffectiveKeyLength") => {
                u32v(if *effective_key_len == 0 { (key.len() * 8) as u32 } else { *effective_key_len })
            }
            (KeyData::Symmetric { iv, alg, .. }, "IV") if alg.block_size() > 0 => iv.clone(),
            (KeyData::Symmetric { alg, .. }, "AlgorithmName") => wide_bytes(alg.as_name()),
            (KeyData::Symmetric { .. }, "PrimitiveType") => u32v(BCRYPT_CIPHER_INTERFACE),
            (KeyData::Rsa(r), "KeyLength" | "KeyStrength") => u32v(r.pub_key.bits() as u32),
            (KeyData::Rsa(r), "BlockLength" | "SignatureLength" | "PublicKeyLength") => u32v(r.pub_key.k() as u32),
            (KeyData::Rsa(_), "KeyLengths") => key_lengths(512, 16384, 64),
            (KeyData::Rsa(_), "PaddingSchemes") => u32v(
                BCRYPT_SUPPORTED_PAD_PKCS1_ENC
                    | BCRYPT_SUPPORTED_PAD_PKCS1_SIG
                    | BCRYPT_SUPPORTED_PAD_OAEP
                    | BCRYPT_SUPPORTED_PAD_PSS,
            ),
            (KeyData::Rsa(_), "AlgorithmName") => wide_bytes("RSA"),
            (KeyData::Rsa(_), "PrimitiveType") => u32v(BCRYPT_ASYMMETRIC_ENCRYPTION_INTERFACE),
            (KeyData::PendingRsa { bits }, "KeyLength" | "KeyStrength") => u32v(*bits),
            (KeyData::PendingRsa { bits }, "BlockLength" | "SignatureLength") => u32v((*bits as usize).div_ceil(8) as u32),
            (KeyData::PendingRsa { .. }, "AlgorithmName") => wide_bytes("RSA"),
            (KeyData::Ecc(e), "KeyLength" | "KeyStrength") => u32v(e.curve.bits()),
            (KeyData::Ecc(e), "SignatureLength") if e.usage == EccUsage::Ecdsa => u32v((e.curve.coord_len() * 2) as u32),
            (KeyData::Ecc(e), "BlockLength") => u32v(e.curve.coord_len() as u32),
            (KeyData::Ecc(e), "KeyLengths") => key_lengths(e.curve.bits(), e.curve.bits(), 0),
            (KeyData::Ecc(e), "ECCCurveName") => wide_bytes(e.curve.win_curve_name()),
            (KeyData::Ecc(e), "AlgorithmName") => wide_bytes(e.curve.alg_name(e.usage)),
            (KeyData::Ecc(e), "PrimitiveType") => u32v(match e.usage {
                EccUsage::Ecdsa => BCRYPT_SIGNATURE_INTERFACE,
                EccUsage::Ecdh => BCRYPT_SECRET_AGREEMENT_INTERFACE,
            }),
            (KeyData::PendingEcc { curve: Some(c), .. }, "KeyLength" | "KeyStrength") => u32v(c.bits()),
            (KeyData::PendingEcc { usage: EccUsage::Ecdsa, curve: Some(c) }, "SignatureLength") => {
                u32v((c.coord_len() * 2) as u32)
            }
            (KeyData::PendingEcc { curve: Some(c), .. }, "ECCCurveName") => wide_bytes(c.win_curve_name()),
            (KeyData::PendingEcc { usage, curve: Some(c) }, "AlgorithmName") => wide_bytes(c.alg_name(*usage)),
            _ => {
                log::debug!("BCryptGetProperty(key, {p:?}) -> STATUS_NOT_SUPPORTED");
                return STATUS_NOT_SUPPORTED;
            }
        };
        return put_property(&value, out, out_len, result);
    }

    STATUS_INVALID_HANDLE
}

pub unsafe extern "win64" fn BCryptCreateHash(
    h: *mut c_void,
    out: *mut *mut c_void,
    _obj: *mut u8,
    _obj_len: u32,
    secret: *mut u8,
    secret_len: u32,
    flags: u32,
) -> i32 {
    let Some(a) = alg(h) else { return STATUS_INVALID_HANDLE };
    if out.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let reusable = a.reusable || (flags & BCRYPT_HASH_REUSABLE_FLAG != 0);
    let (kind, key) = match a.kind {
        AlgKind::Hash(d) => (HashKind::Digest(d), a.hmac.then(|| bytes(secret, secret_len).to_vec())),
        AlgKind::AesCmac => {
            if secret.is_null() || !matches!(secret_len, 16 | 24 | 32) {
                return STATUS_INVALID_PARAMETER;
            }
            (HashKind::AesCmac, Some(bytes(secret, secret_len).to_vec()))
        }
        AlgKind::AesGmac => {
            if secret.is_null() || !matches!(secret_len, 16 | 24 | 32) {
                return STATUS_INVALID_PARAMETER;
            }
            (HashKind::AesGmac, Some(bytes(secret, secret_len).to_vec()))
        }
        _ => return STATUS_INVALID_HANDLE,
    };
    std::ptr::write_unaligned(
        out,
        Box::into_raw(Box::new(Hash { magic: HASH_MAGIC, kind, key, iv: vec![0u8; 12], reusable, data: Vec::new() })) as *mut c_void,
    );
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn BCryptHashData(h: *mut c_void, input: *mut u8, input_len: u32, _flags: u32) -> i32 {
    let Some(o) = hash_obj(h) else { return STATUS_INVALID_HANDLE };
    if input.is_null() && input_len != 0 {
        return STATUS_INVALID_PARAMETER;
    }
    o.data.extend_from_slice(bytes(input, input_len));
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn BCryptFinishHash(h: *mut c_void, out: *mut u8, out_len: u32, _flags: u32) -> i32 {
    let Some(o) = hash_obj(h) else { return STATUS_INVALID_HANDLE };
    if out.is_null() || out_len as usize != o.kind.len() {
        return STATUS_INVALID_PARAMETER;
    }
    let d = match o.kind {
        HashKind::Digest(digest) => match &o.key {
            Some(k) => digest.hmac(k, &o.data),
            None => digest.hash(&o.data),
        },
        HashKind::AesCmac => {
            let Some(ref k) = o.key else { return STATUS_INVALID_HANDLE };
            match aes_cmac(k, &o.data) {
                Ok(tag) => tag.to_vec(),
                Err(e) => return e,
            }
        }
        HashKind::AesGmac => {
            let Some(ref k) = o.key else { return STATUS_INVALID_HANDLE };
            match aes_gcm_crypt(true, k, &o.iv, &o.data, &[], 16, None) {
                Ok((_, tag)) => tag,
                Err(e) => return e,
            }
        }
    };
    std::ptr::copy_nonoverlapping(d.as_ptr(), out, d.len());
    o.data.clear();
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn BCryptDestroyHash(h: *mut c_void) -> i32 {
    if hash_obj(h).is_none() {
        return STATUS_INVALID_HANDLE;
    }
    let mut o = Box::from_raw(h as *mut Hash);
    o.magic = 0;
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn BCryptImportKeyPair(
    h: *mut c_void,
    import_key: *mut c_void,
    blob_type: *const u16,
    out: *mut *mut c_void,
    input: *mut u8,
    input_len: u32,
    _flags: u32,
) -> i32 {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    let Some(a) = alg(h) else { return STATUS_INVALID_HANDLE };
    if out.is_null() || blob_type.is_null() || input.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    if !import_key.is_null() {
        // Microsoft Learn (BCryptImportKeyPair): "hImportKey: This parameter is not currently used and should be NULL."
        return STATUS_NOT_SUPPORTED;
    }
    let t = crate::strings::from_wide(blob_type);
    let blob = bytes(input, input_len);
    match a.kind {
        AlgKind::Rsa => {
            if !matches!(
                t.as_str(),
                "RSAPUBLICBLOB" | "RSAPRIVATEBLOB" | "RSAFULLPRIVATEBLOB" | "PUBLICBLOB" | "PRIVATEBLOB" | "CAPIPUBLICBLOB" | "CAPIPRIVATEBLOB"
            ) {
                log::debug!("BCryptImportKeyPair({}, {t:?}) -> STATUS_NOT_SUPPORTED", a.name);
                return STATUS_NOT_SUPPORTED;
            }
            let Some(rsa) = RsaKey::from_blob(&t, blob) else {
                return STATUS_INVALID_PARAMETER;
            };
            log::debug!("BCryptImportKeyPair({t}): {}-bit RSA modulus", rsa.pub_key.bits());
            std::ptr::write_unaligned(
                out,
                Box::into_raw(Box::new(Key { magic: KEY_MAGIC, data: KeyData::Rsa(rsa) })) as *mut c_void,
            );
            STATUS_SUCCESS
        }
        AlgKind::Ecc { usage, curve: alg_curve } => {
            if !matches!(t.as_str(), "ECCPUBLICBLOB" | "ECCPRIVATEBLOB" | "PUBLICBLOB" | "PRIVATEBLOB") {
                log::debug!("BCryptImportKeyPair({}, {t:?}) -> STATUS_NOT_SUPPORTED", a.name);
                return STATUS_NOT_SUPPORTED;
            }
            let expected = alg_curve.or(a.ecc_curve);
            let Some(ec) = EccKey::from_blob(&t, usage, expected, blob) else {
                return STATUS_INVALID_PARAMETER;
            };
            log::debug!("BCryptImportKeyPair({t}): {:?} {:?} key (priv={})", ec.usage, ec.curve, ec.d.is_some());
            std::ptr::write_unaligned(
                out,
                Box::into_raw(Box::new(Key { magic: KEY_MAGIC, data: KeyData::Ecc(ec) })) as *mut c_void,
            );
            STATUS_SUCCESS
        }
        _ => {
            // Non-asymmetric algorithm handles do not support BCryptImportKeyPair on Windows.
            log::debug!("BCryptImportKeyPair({}, {t:?}) -> STATUS_NOT_SUPPORTED", a.name);
            STATUS_NOT_SUPPORTED
        }
    }
}

pub unsafe extern "win64" fn BCryptDestroyKey(h: *mut c_void) -> i32 {
    if key(h).is_none() {
        return STATUS_INVALID_HANDLE;
    }
    let mut k = Box::from_raw(h as *mut Key);
    k.magic = 0;
    STATUS_SUCCESS
}

unsafe fn parse_rsa_padding(padding: *mut c_void, flags: u32, hash_len: usize) -> Result<(Option<Digest>, Option<usize>, bool), i32> {
    let optional_oid = (flags & BCRYPT_PAD_PKCS1_OPTIONAL_HASH_OID) != 0;
    let pad_flags = flags & !BCRYPT_PAD_PKCS1_OPTIONAL_HASH_OID;
    let alg_id = if padding.is_null() {
        None
    } else {
        let alg_ptr = std::ptr::read_unaligned(padding as *const *const u16);
        crate::strings::from_wide_opt(alg_ptr)
    };
    let digest = match alg_id.as_deref().map(Digest::from_name) {
        None => None,
        Some(Some(d)) => Some(d),
        Some(None) => return Err(STATUS_NOT_SUPPORTED),
    };
    if digest.is_some_and(|d| d.len() != hash_len) {
        return Err(STATUS_INVALID_PARAMETER);
    }
    if pad_flags == BCRYPT_PAD_PSS {
        if optional_oid || digest.is_none() || padding.is_null() {
            return Err(STATUS_INVALID_PARAMETER);
        }
        let salt = std::ptr::read_unaligned((padding as *const u8).add(8) as *const u32) as usize;
        Ok((digest, Some(salt), false))
    } else if pad_flags == BCRYPT_PAD_PKCS1 || (pad_flags == 0 && optional_oid) {
        Ok((digest, None, optional_oid))
    } else {
        Err(STATUS_INVALID_PARAMETER)
    }
}

#[allow(clippy::too_many_arguments)]
pub unsafe extern "win64" fn BCryptSignHash(
    h_key: *mut c_void,
    padding: *mut c_void,
    input: *const u8,
    input_len: u32,
    output: *mut u8,
    output_len: u32,
    result_len: *mut u32,
    flags: u32,
) -> i32 {
    if result_len.is_null() || input.is_null() || input_len == 0 {
        return STATUS_INVALID_PARAMETER;
    }
    let Some(k) = key(h_key) else { return STATUS_INVALID_HANDLE };
    let hash = bytes(input, input_len);

    match &k.data {
        KeyData::Rsa(rsa) => {
            if !rsa.has_private() {
                return STATUS_INVALID_HANDLE;
            }
            let sig_len = rsa.pub_key.k();
            if output.is_null() {
                std::ptr::write_unaligned(result_len, sig_len as u32);
                return STATUS_SUCCESS;
            }
            if (output_len as usize) < sig_len {
                std::ptr::write_unaligned(result_len, sig_len as u32);
                return STATUS_BUFFER_TOO_SMALL;
            }
            let (digest, pss_salt, _) = match parse_rsa_padding(padding, flags, hash.len()) {
                Ok(v) => v,
                Err(e) => return e,
            };
            let sig = match pss_salt {
                Some(salt) => rsa.sign_pss(digest.unwrap(), hash, salt),
                None => rsa.sign_pkcs1(digest, hash),
            };
            let sig = match sig {
                Ok(s) => s,
                Err(e) => return e,
            };
            std::ptr::copy_nonoverlapping(sig.as_ptr(), output, sig.len());
            std::ptr::write_unaligned(result_len, sig.len() as u32);
            STATUS_SUCCESS
        }
        KeyData::Ecc(ec) if ec.usage == EccUsage::Ecdsa => {
            if ec.d.is_none() {
                return STATUS_INVALID_HANDLE;
            }
            if !padding.is_null() || flags != 0 {
                return STATUS_INVALID_PARAMETER;
            }
            let sig_len = ec.curve.coord_len() * 2;
            if output.is_null() {
                std::ptr::write_unaligned(result_len, sig_len as u32);
                return STATUS_SUCCESS;
            }
            if (output_len as usize) < sig_len {
                std::ptr::write_unaligned(result_len, sig_len as u32);
                return STATUS_BUFFER_TOO_SMALL;
            }
            let sig = match ec.sign_hash(hash) {
                Ok(s) => s,
                Err(e) => return e,
            };
            std::ptr::copy_nonoverlapping(sig.as_ptr(), output, sig.len());
            std::ptr::write_unaligned(result_len, sig.len() as u32);
            STATUS_SUCCESS
        }
        _ => STATUS_INVALID_HANDLE,
    }
}

pub unsafe extern "win64" fn BCryptVerifySignature(
    h: *mut c_void,
    padding: *mut c_void,
    hash: *mut u8,
    hash_len: u32,
    sig: *mut u8,
    sig_len: u32,
    flags: u32,
) -> i32 {
    let Some(k) = key(h) else { return STATUS_INVALID_HANDLE };
    if hash.is_null() || sig.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let (hash_b, sig_b) = (bytes(hash, hash_len), bytes(sig, sig_len));

    match &k.data {
        KeyData::Rsa(rsa) => {
            let (digest, pss_salt, optional_oid) = match parse_rsa_padding(padding, flags, hash_b.len()) {
                Ok(v) => v,
                Err(e) => return e,
            };
            if sig_b.len() != rsa.pub_key.k() {
                return STATUS_INVALID_BUFFER_SIZE;
            }
            let ok = match pss_salt {
                Some(salt) => rsa.pub_key.verify_pss(digest.unwrap(), hash_b, salt, sig_b),
                None => {
                    rsa.pub_key.verify_pkcs1(digest, hash_b, sig_b)
                        || (optional_oid && digest.is_some() && rsa.pub_key.verify_pkcs1(None, hash_b, sig_b))
                }
            };
            log::debug!("BCryptVerifySignature(RSA, flags 0x{flags:x}) -> {}", if ok { "valid" } else { "INVALID" });
            if ok { STATUS_SUCCESS } else { STATUS_INVALID_SIGNATURE }
        }
        KeyData::Ecc(ec) if ec.usage == EccUsage::Ecdsa => {
            if !padding.is_null() || flags != 0 {
                return STATUS_INVALID_PARAMETER;
            }
            if sig_b.len() != ec.curve.coord_len() * 2 {
                return STATUS_INVALID_SIGNATURE;
            }
            let ok = match ec.verify_hash(hash_b, sig_b) {
                Ok(v) => v,
                Err(e) => return e,
            };
            log::debug!("BCryptVerifySignature({:?}) -> {}", ec.curve, if ok { "valid" } else { "INVALID" });
            if ok { STATUS_SUCCESS } else { STATUS_INVALID_SIGNATURE }
        }
        _ => STATUS_INVALID_HANDLE,
    }
}

unsafe fn rsa_crypt(
    encrypt: bool,
    rsa: &RsaKey,
    input: &[u8],
    padding_info: *mut c_void,
    output: *mut u8,
    output_len: u32,
    result_len: *mut u32,
    flags: u32,
) -> i32 {
    if !encrypt && !rsa.has_private() {
        return STATUS_INVALID_HANDLE;
    }
    let k = rsa.k();
    // Size query for encrypt always returns modulus size k without doing modexp.
    if encrypt && output.is_null() {
        std::ptr::write_unaligned(result_len, k as u32);
        return STATUS_SUCCESS;
    }
    let res = match flags {
        BCRYPT_PAD_NONE => {
            if encrypt {
                rsa.encrypt_raw(input)
            } else {
                rsa.decrypt_raw(input)
            }
        }
        BCRYPT_PAD_PKCS1 => {
            if encrypt {
                rsa.encrypt_pkcs1(input)
            } else {
                rsa.decrypt_pkcs1(input)
            }
        }
        BCRYPT_PAD_OAEP => {
            let (digest, label) = if padding_info.is_null() {
                (Digest::Sha1, &[][..])
            } else {
                let info = std::ptr::read_unaligned(padding_info as *const BcryptOaepPaddingInfo);
                let d = match crate::strings::from_wide_opt(info.psz_alg_id) {
                    None => Digest::Sha1,
                    Some(name) => match Digest::from_name(&name) {
                        Some(d) => d,
                        None => return STATUS_NOT_SUPPORTED,
                    },
                };
                let lbl = if info.pb_label.is_null() || info.cb_label == 0 {
                    &[][..]
                } else {
                    std::slice::from_raw_parts(info.pb_label, info.cb_label as usize)
                };
                (d, lbl)
            };
            if encrypt {
                rsa.encrypt_oaep(digest, label, input)
            } else {
                rsa.decrypt_oaep(digest, label, input)
            }
        }
        _ => return STATUS_INVALID_PARAMETER,
    };
    let out_bytes = match res {
        Ok(v) => v,
        Err(e) => return e,
    };
    if output.is_null() {
        std::ptr::write_unaligned(result_len, out_bytes.len() as u32);
        return STATUS_SUCCESS;
    }
    if (output_len as usize) < out_bytes.len() {
        std::ptr::write_unaligned(result_len, out_bytes.len() as u32);
        return STATUS_BUFFER_TOO_SMALL;
    }
    if !out_bytes.is_empty() {
        std::ptr::copy_nonoverlapping(out_bytes.as_ptr(), output, out_bytes.len());
    }
    std::ptr::write_unaligned(result_len, out_bytes.len() as u32);
    STATUS_SUCCESS
}

#[allow(clippy::too_many_arguments)]
unsafe fn bcrypt_crypt(
    encrypt: bool,
    h_key: *mut c_void,
    input: *const u8,
    input_len: u32,
    padding_info: *mut c_void,
    iv: *mut u8,
    iv_len: u32,
    output: *mut u8,
    output_len: u32,
    result_len: *mut u32,
    flags: u32,
) -> i32 {
    let Some(k) = key_mut(h_key) else {
        return STATUS_INVALID_HANDLE;
    };
    if result_len.is_null() || (input_len > 0 && input.is_null()) {
        return STATUS_INVALID_PARAMETER;
    }
    let in_slice = bytes(input, input_len);

    if let KeyData::Rsa(ref rsa) = k.data {
        return rsa_crypt(encrypt, rsa, in_slice, padding_info, output, output_len, result_len, flags);
    }

    let KeyData::Symmetric {
        alg,
        ref key,
        mode,
        msg_block_len,
        effective_key_len,
        iv: ref mut key_iv,
        ref mut rc4,
    } = k.data
    else {
        // ECDSA / ECDH / unfinalized keys cannot be used with BCryptEncrypt / BCryptDecrypt.
        return STATUS_NOT_SUPPORTED;
    };

    let bs = alg.block_size();
    let pkcs7 = (flags & BCRYPT_BLOCK_PADDING) != 0;

    // Size-query path (output == NULL): return exact required output buffer size without mutating state.
    if output.is_null() {
        let need = match (alg, mode) {
            (SymAlg::Rc4, _) => in_slice.len(),
            (_, CipherMode::Ecb | CipherMode::Cbc) => {
                if encrypt && pkcs7 {
                    ((in_slice.len() / bs) + 1) * bs
                } else {
                    if in_slice.len() % bs != 0 {
                        return STATUS_INVALID_BUFFER_SIZE;
                    }
                    in_slice.len()
                }
            }
            (_, CipherMode::Cfb | CipherMode::Ctr | CipherMode::Gcm | CipherMode::Ccm | CipherMode::Na) => {
                in_slice.len()
            }
        };
        std::ptr::write_unaligned(result_len, need as u32);
        return STATUS_SUCCESS;
    }

    if alg == SymAlg::Rc4 {
        if (output_len as usize) < in_slice.len() {
            std::ptr::write_unaligned(result_len, in_slice.len() as u32);
            return STATUS_BUFFER_TOO_SMALL;
        }
        let state = rc4.get_or_insert_with(|| Rc4State::new(key));
        let out_bytes = state.apply(in_slice);
        if !out_bytes.is_empty() {
            std::ptr::copy_nonoverlapping(out_bytes.as_ptr(), output, out_bytes.len());
        }
        std::ptr::write_unaligned(result_len, out_bytes.len() as u32);
        return STATUS_SUCCESS;
    }

    let out_bytes = match mode {
        CipherMode::Gcm | CipherMode::Ccm => {
            if alg != SymAlg::Aes || padding_info.is_null() {
                return STATUS_INVALID_PARAMETER;
            }
            let info = std::ptr::read_unaligned(padding_info as *const BcryptAuthCipherModeInfo);
            let nonce = bytes(info.pb_nonce, info.cb_nonce);
            let aad = bytes(info.pb_auth_data, info.cb_auth_data);
            let tag_len = info.cb_tag as usize;
            if info.pb_tag.is_null() || tag_len == 0 {
                return STATUS_INVALID_PARAMETER;
            }
            let expected_tag = (!encrypt).then(|| bytes(info.pb_tag, info.cb_tag));
            let res = if mode == CipherMode::Gcm {
                aes_gcm_crypt(encrypt, key, nonce, aad, in_slice, tag_len, expected_tag)
            } else {
                aes_ccm_crypt(encrypt, key, nonce, aad, in_slice, tag_len, expected_tag)
            };
            let (v, tag) = match res {
                Ok(r) => r,
                Err(e) => return e,
            };
            if (output_len as usize) < v.len() {
                std::ptr::write_unaligned(result_len, v.len() as u32);
                return STATUS_BUFFER_TOO_SMALL;
            }
            if encrypt {
                std::ptr::copy_nonoverlapping(tag.as_ptr(), info.pb_tag, tag.len());
            }
            v
        }
        CipherMode::Ecb | CipherMode::Cbc | CipherMode::Cfb | CipherMode::Ctr => {
            let mut work_iv = if !iv.is_null() {
                if (iv_len as usize) < bs {
                    return STATUS_INVALID_PARAMETER;
                }
                std::slice::from_raw_parts(iv, bs).to_vec()
            } else {
                if key_iv.len() < bs {
                    key_iv.resize(bs, 0);
                }
                key_iv[..bs].to_vec()
            };
            let v = match sym_crypt_block_mode(
                alg,
                mode,
                encrypt,
                pkcs7,
                key,
                effective_key_len,
                &mut work_iv,
                msg_block_len as usize,
                in_slice,
            ) {
                Ok(v) => v,
                Err(e) => return e,
            };
            if (output_len as usize) < v.len() {
                std::ptr::write_unaligned(result_len, v.len() as u32);
                return STATUS_BUFFER_TOO_SMALL;
            }
            if mode != CipherMode::Ecb {
                if !iv.is_null() {
                    std::ptr::copy_nonoverlapping(work_iv.as_ptr(), iv, bs);
                } else {
                    key_iv[..bs].copy_from_slice(&work_iv);
                }
            }
            v
        }
        CipherMode::Na => return STATUS_NOT_SUPPORTED,
    };

    if (output_len as usize) < out_bytes.len() {
        std::ptr::write_unaligned(result_len, out_bytes.len() as u32);
        return STATUS_BUFFER_TOO_SMALL;
    }
    if !out_bytes.is_empty() {
        std::ptr::copy_nonoverlapping(out_bytes.as_ptr(), output, out_bytes.len());
    }
    std::ptr::write_unaligned(result_len, out_bytes.len() as u32);
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn BCryptDecrypt(
    h_key: *mut c_void,
    input: *const u8,
    input_len: u32,
    padding_info: *mut c_void,
    iv: *mut u8,
    iv_len: u32,
    output: *mut u8,
    output_len: u32,
    result_len: *mut u32,
    flags: u32,
) -> i32 {
    bcrypt_crypt(false, h_key, input, input_len, padding_info, iv, iv_len, output, output_len, result_len, flags)
}

pub unsafe extern "win64" fn BCryptEncrypt(
    h_key: *mut c_void,
    input: *const u8,
    input_len: u32,
    padding_info: *mut c_void,
    iv: *mut u8,
    iv_len: u32,
    output: *mut u8,
    output_len: u32,
    result_len: *mut u32,
    flags: u32,
) -> i32 {
    bcrypt_crypt(true, h_key, input, input_len, padding_info, iv, iv_len, output, output_len, result_len, flags)
}

#[allow(clippy::too_many_arguments)]
pub unsafe extern "win64" fn BCryptImportKey(
    h_alg: *mut c_void,
    h_import_key: *mut c_void,
    blob_type: *const u16,
    ph_key: *mut *mut c_void,
    _key_obj: *mut u8,
    _key_obj_len: u32,
    input: *const u8,
    input_len: u32,
    _flags: u32,
) -> i32 {
    if !ph_key.is_null() {
        std::ptr::write_unaligned(ph_key, std::ptr::null_mut());
    }
    let Some(a) = alg(h_alg) else {
        return STATUS_INVALID_HANDLE;
    };
    if ph_key.is_null() || blob_type.is_null() || input.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let AlgKind::Symmetric(sym_alg) = a.kind else {
        return STATUS_NOT_SUPPORTED;
    };
    let t = crate::strings::from_wide(blob_type);
    let blob = bytes(input, input_len);

    if t == "Rfc3565KeyWrapBlob" {
        let Some(imp_k) = key(h_import_key) else {
            return STATUS_INVALID_HANDLE;
        };
        let KeyData::Symmetric { alg: SymAlg::Aes, key: ref kek, .. } = imp_k.data else {
            return STATUS_NOT_SUPPORTED;
        };
        let raw_key = match aes_key_unwrap(kek, blob) {
            Ok(k) => k,
            Err(e) => return e,
        };
        if !sym_alg.valid_key_len(raw_key.len()) {
            return STATUS_INVALID_PARAMETER;
        }
        let k = Key {
            magic: KEY_MAGIC,
            data: KeyData::Symmetric {
                alg: sym_alg,
                rc4: (sym_alg == SymAlg::Rc4).then(|| Rc4State::new(&raw_key)),
                key: raw_key,
                mode: a.mode,
                msg_block_len: a.msg_block_len,
                effective_key_len: a.effective_key_len,
                iv: vec![0u8; sym_alg.block_size()],
            },
        };
        std::ptr::write_unaligned(ph_key, Box::into_raw(Box::new(k)) as *mut c_void);
        return STATUS_SUCCESS;
    }

    if !h_import_key.is_null() {
        // Non-wrap blob types do not take a wrapping key handle in CNG.
        return STATUS_NOT_SUPPORTED;
    }

    if t == "KeyDataBlob" {
        if blob.len() < 12 {
            return STATUS_INVALID_PARAMETER;
        }
        let magic = u32::from_le_bytes(blob[0..4].try_into().unwrap());
        let version = u32::from_le_bytes(blob[4..8].try_into().unwrap());
        let cb_key = u32::from_le_bytes(blob[8..12].try_into().unwrap()) as usize;
        if magic != BCRYPT_KEY_DATA_BLOB_MAGIC || version != 1 || blob.len() < 12 + cb_key || !sym_alg.valid_key_len(cb_key) {
            return STATUS_INVALID_PARAMETER;
        }
        let raw = blob[12..12 + cb_key].to_vec();
        let k = Key {
            magic: KEY_MAGIC,
            data: KeyData::Symmetric {
                alg: sym_alg,
                rc4: (sym_alg == SymAlg::Rc4).then(|| Rc4State::new(&raw)),
                key: raw,
                mode: a.mode,
                msg_block_len: a.msg_block_len,
                effective_key_len: a.effective_key_len,
                iv: vec![0u8; sym_alg.block_size()],
            },
        };
        std::ptr::write_unaligned(ph_key, Box::into_raw(Box::new(k)) as *mut c_void);
        return STATUS_SUCCESS;
    }

    if t == "OpaqueKeyBlob" {
        if blob.len() < 20 {
            return STATUS_INVALID_PARAMETER;
        }
        let magic = u32::from_le_bytes(blob[0..4].try_into().unwrap());
        let version = u32::from_le_bytes(blob[4..8].try_into().unwrap());
        if magic != BCRYPT_OPAQUE_KEY_MAGIC {
            return STATUS_INVALID_PARAMETER;
        }
        if version == 1 && sym_alg == SymAlg::Aes {
            let mode_raw = u32::from_le_bytes(blob[8..12].try_into().unwrap());
            let mbl = u32::from_le_bytes(blob[12..16].try_into().unwrap());
            let cb_key = u32::from_le_bytes(blob[16..20].try_into().unwrap()) as usize;
            let Some(mode) = CipherMode::from_u32(mode_raw) else {
                return STATUS_INVALID_PARAMETER;
            };
            if !(1..=16).contains(&mbl) || blob.len() < 20 + cb_key || !sym_alg.valid_key_len(cb_key) {
                return STATUS_INVALID_PARAMETER;
            }
            let k = Key {
                magic: KEY_MAGIC,
                data: KeyData::Symmetric {
                    alg: SymAlg::Aes,
                    key: blob[20..20 + cb_key].to_vec(),
                    mode,
                    msg_block_len: mbl,
                    effective_key_len: 0,
                    iv: vec![0u8; 16],
                    rc4: None,
                },
            };
            std::ptr::write_unaligned(ph_key, Box::into_raw(Box::new(k)) as *mut c_void);
            return STATUS_SUCCESS;
        }
        if version == 2 && blob.len() >= 28 {
            let alg_raw = u32::from_le_bytes(blob[8..12].try_into().unwrap());
            let mode_raw = u32::from_le_bytes(blob[12..16].try_into().unwrap());
            let mbl = u32::from_le_bytes(blob[16..20].try_into().unwrap());
            let eff = u32::from_le_bytes(blob[20..24].try_into().unwrap());
            let cb_key = u32::from_le_bytes(blob[24..28].try_into().unwrap()) as usize;
            let Some(blob_alg) = SymAlg::from_u32(alg_raw) else {
                return STATUS_INVALID_PARAMETER;
            };
            let Some(mode) = CipherMode::from_u32(mode_raw) else {
                return STATUS_INVALID_PARAMETER;
            };
            if blob_alg != sym_alg || !sym_alg.supports_mode(mode) || blob.len() < 28 + cb_key || !sym_alg.valid_key_len(cb_key) {
                return STATUS_INVALID_PARAMETER;
            }
            let raw = blob[28..28 + cb_key].to_vec();
            let k = Key {
                magic: KEY_MAGIC,
                data: KeyData::Symmetric {
                    alg: sym_alg,
                    rc4: (sym_alg == SymAlg::Rc4).then(|| Rc4State::new(&raw)),
                    key: raw,
                    mode,
                    msg_block_len: mbl,
                    effective_key_len: eff,
                    iv: vec![0u8; sym_alg.block_size()],
                },
            };
            std::ptr::write_unaligned(ph_key, Box::into_raw(Box::new(k)) as *mut c_void);
            return STATUS_SUCCESS;
        }
        return STATUS_INVALID_PARAMETER;
    }

    log::debug!("BCryptImportKey({}, {t:?}) -> STATUS_NOT_SUPPORTED", a.name);
    STATUS_NOT_SUPPORTED
}

pub unsafe extern "win64" fn BCryptExportKey(
    h_key: *mut c_void,
    h_export_key: *mut c_void,
    blob_type: *const u16,
    output: *mut u8,
    output_len: u32,
    result_len: *mut u32,
    _flags: u32,
) -> i32 {
    if result_len.is_null() || blob_type.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let Some(k) = key(h_key) else {
        return STATUS_INVALID_HANDLE;
    };
    let t = crate::strings::from_wide(blob_type);

    if t == "Rfc3565KeyWrapBlob" {
        let Some(exp_k) = key(h_export_key) else {
            std::ptr::write_unaligned(result_len, 0);
            return STATUS_INVALID_HANDLE;
        };
        let KeyData::Symmetric { alg: SymAlg::Aes, key: ref kek, .. } = exp_k.data else {
            std::ptr::write_unaligned(result_len, 0);
            return STATUS_NOT_SUPPORTED;
        };
        let KeyData::Symmetric { key: ref raw_key, .. } = k.data else {
            std::ptr::write_unaligned(result_len, 0);
            return STATUS_NOT_SUPPORTED;
        };
        let wrapped = match aes_key_wrap(kek, raw_key) {
            Ok(w) => w,
            Err(e) => {
                std::ptr::write_unaligned(result_len, 0);
                return e;
            }
        };
        return put_property(&wrapped, output, output_len, result_len);
    }

    if !h_export_key.is_null() {
        std::ptr::write_unaligned(result_len, 0);
        return STATUS_NOT_SUPPORTED;
    }

    match (&k.data, t.as_str()) {
        (KeyData::Symmetric { key, .. }, "KeyDataBlob") => {
            let mut blob = Vec::with_capacity(12 + key.len());
            blob.extend_from_slice(&BCRYPT_KEY_DATA_BLOB_MAGIC.to_le_bytes());
            blob.extend_from_slice(&1u32.to_le_bytes());
            blob.extend_from_slice(&(key.len() as u32).to_le_bytes());
            blob.extend_from_slice(key);
            return put_property(&blob, output, output_len, result_len);
        }
        (KeyData::Symmetric { alg: SymAlg::Aes, key, mode, msg_block_len, .. }, "OpaqueKeyBlob") => {
            let mut blob = Vec::with_capacity(20 + key.len());
            blob.extend_from_slice(&BCRYPT_OPAQUE_KEY_MAGIC.to_le_bytes());
            blob.extend_from_slice(&1u32.to_le_bytes());
            blob.extend_from_slice(&mode.to_u32().to_le_bytes());
            blob.extend_from_slice(&msg_block_len.to_le_bytes());
            blob.extend_from_slice(&(key.len() as u32).to_le_bytes());
            blob.extend_from_slice(key);
            return put_property(&blob, output, output_len, result_len);
        }
        (KeyData::Symmetric { alg, key, mode, msg_block_len, effective_key_len, .. }, "OpaqueKeyBlob") => {
            let mut blob = Vec::with_capacity(28 + key.len());
            blob.extend_from_slice(&BCRYPT_OPAQUE_KEY_MAGIC.to_le_bytes());
            blob.extend_from_slice(&2u32.to_le_bytes());
            blob.extend_from_slice(&alg.to_u32().to_le_bytes());
            blob.extend_from_slice(&mode.to_u32().to_le_bytes());
            blob.extend_from_slice(&msg_block_len.to_le_bytes());
            blob.extend_from_slice(&effective_key_len.to_le_bytes());
            blob.extend_from_slice(&(key.len() as u32).to_le_bytes());
            blob.extend_from_slice(key);
            return put_property(&blob, output, output_len, result_len);
        }
        (KeyData::Rsa(rsa), "RSAPUBLICBLOB" | "PUBLICBLOB") => {
            let blob = rsa.to_public_blob();
            return put_property(&blob, output, output_len, result_len);
        }
        (KeyData::Rsa(rsa), "RSAPRIVATEBLOB" | "PRIVATEBLOB") => {
            let Some(blob) = rsa.to_private_blob(false) else {
                return STATUS_NOT_SUPPORTED;
            };
            return put_property(&blob, output, output_len, result_len);
        }
        (KeyData::Rsa(rsa), "RSAFULLPRIVATEBLOB") => {
            let Some(blob) = rsa.to_private_blob(true) else {
                return STATUS_NOT_SUPPORTED;
            };
            return put_property(&blob, output, output_len, result_len);
        }
        (KeyData::Rsa(rsa), "CAPIPUBLICBLOB") => {
            let Some(blob) = rsa.to_capi_public_blob(CAPI_CALG_RSA_KEYX) else {
                return STATUS_INVALID_PARAMETER;
            };
            return put_property(&blob, output, output_len, result_len);
        }
        (KeyData::Rsa(rsa), "CAPIPRIVATEBLOB") => {
            let Some(blob) = rsa.to_capi_private_blob(CAPI_CALG_RSA_KEYX) else {
                return STATUS_NOT_SUPPORTED;
            };
            return put_property(&blob, output, output_len, result_len);
        }
        (KeyData::Ecc(ec), "ECCPUBLICBLOB" | "PUBLICBLOB") => {
            let blob = ec.to_public_blob();
            return put_property(&blob, output, output_len, result_len);
        }
        (KeyData::Ecc(ec), "ECCPRIVATEBLOB" | "PRIVATEBLOB") => {
            let Some(blob) = ec.to_private_blob() else {
                return STATUS_NOT_SUPPORTED;
            };
            return put_property(&blob, output, output_len, result_len);
        }
        (KeyData::PendingRsa { .. } | KeyData::PendingEcc { .. }, _) => {
            return STATUS_INVALID_HANDLE;
        }
        _ => {}
    }
    std::ptr::write_unaligned(result_len, 0);
    log::debug!("BCryptExportKey({t:?}) -> STATUS_NOT_SUPPORTED");
    STATUS_NOT_SUPPORTED
}

pub unsafe extern "win64" fn BCryptGenerateSymmetricKey(
    h_alg: *mut c_void,
    ph_key: *mut *mut c_void,
    _key_obj: *mut u8,
    _key_obj_len: u32,
    secret: *const u8,
    secret_len: u32,
    _flags: u32,
) -> i32 {
    if !ph_key.is_null() {
        std::ptr::write_unaligned(ph_key, std::ptr::null_mut());
    }
    let Some(a) = alg(h_alg) else {
        return STATUS_INVALID_HANDLE;
    };
    if ph_key.is_null() || secret.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let AlgKind::Symmetric(sym_alg) = a.kind else {
        // Non-symmetric algorithm handle: Windows returns STATUS_NOT_SUPPORTED.
        log::debug!("BCryptGenerateSymmetricKey({}) -> STATUS_NOT_SUPPORTED", a.name);
        return STATUS_NOT_SUPPORTED;
    };
    if !sym_alg.valid_key_len(secret_len as usize) {
        return STATUS_INVALID_PARAMETER;
    }
    let raw = bytes(secret, secret_len).to_vec();
    let k = Key {
        magic: KEY_MAGIC,
        data: KeyData::Symmetric {
            alg: sym_alg,
            rc4: (sym_alg == SymAlg::Rc4).then(|| Rc4State::new(&raw)),
            key: raw,
            mode: a.mode,
            msg_block_len: a.msg_block_len,
            effective_key_len: a.effective_key_len,
            iv: vec![0u8; sym_alg.block_size()],
        },
    };
    std::ptr::write_unaligned(ph_key, Box::into_raw(Box::new(k)) as *mut c_void);
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn BCryptDuplicateKey(
    h_key: *mut c_void,
    ph_new_key: *mut *mut c_void,
    _key_obj: *mut u8,
    _key_obj_len: u32,
    _flags: u32,
) -> i32 {
    if ph_new_key.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let Some(k) = key(h_key) else {
        return STATUS_INVALID_HANDLE;
    };
    let dup = match &k.data {
        KeyData::Rsa(r) => KeyData::Rsa(r.clone()),
        KeyData::PendingRsa { bits } => KeyData::PendingRsa { bits: *bits },
        KeyData::Ecc(e) => KeyData::Ecc(e.clone()),
        KeyData::PendingEcc { usage, curve } => KeyData::PendingEcc { usage: *usage, curve: *curve },
        KeyData::Symmetric { alg, key, mode, msg_block_len, effective_key_len, iv, rc4 } => KeyData::Symmetric {
            alg: *alg,
            key: key.clone(),
            mode: *mode,
            msg_block_len: *msg_block_len,
            effective_key_len: *effective_key_len,
            iv: iv.clone(),
            rc4: rc4.clone(),
        },
    };
    std::ptr::write_unaligned(ph_new_key, Box::into_raw(Box::new(Key { magic: KEY_MAGIC, data: dup })) as *mut c_void);
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn BCryptGenerateKeyPair(
    h_alg: *mut c_void,
    ph_key: *mut *mut c_void,
    length: u32,
    _flags: u32,
) -> i32 {
    if !ph_key.is_null() {
        std::ptr::write_unaligned(ph_key, std::ptr::null_mut());
    }
    let Some(a) = alg(h_alg) else {
        return STATUS_INVALID_HANDLE;
    };
    if ph_key.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let data = match a.kind {
        AlgKind::Rsa => {
            if !(1024..=4096).contains(&length) || length % 64 != 0 {
                return STATUS_INVALID_PARAMETER;
            }
            KeyData::PendingRsa { bits: length }
        }
        AlgKind::Ecc { usage, curve: Some(curve) } => {
            if length != 0 && length != curve.bits() {
                return STATUS_INVALID_PARAMETER;
            }
            KeyData::PendingEcc { usage, curve: Some(curve) }
        }
        AlgKind::Ecc { usage, curve: None } => {
            let curve = if length != 0 {
                let Some(c) = EccCurve::from_bits(length) else {
                    return STATUS_INVALID_PARAMETER;
                };
                Some(c)
            } else {
                a.ecc_curve
            };
            KeyData::PendingEcc { usage, curve }
        }
        _ => {
            log::debug!("BCryptGenerateKeyPair({}) -> STATUS_NOT_SUPPORTED", a.name);
            return STATUS_NOT_SUPPORTED;
        }
    };
    std::ptr::write_unaligned(ph_key, Box::into_raw(Box::new(Key { magic: KEY_MAGIC, data })) as *mut c_void);
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn BCryptFinalizeKeyPair(h_key: *mut c_void, _flags: u32) -> i32 {
    let Some(k) = key_mut(h_key) else {
        return STATUS_INVALID_HANDLE;
    };
    match k.data {
        KeyData::PendingRsa { bits } => match RsaKey::generate(bits) {
            Ok(rsa) => {
                k.data = KeyData::Rsa(rsa);
                STATUS_SUCCESS
            }
            Err(e) => e,
        },
        KeyData::PendingEcc { usage, curve } => {
            let Some(c) = curve else {
                return STATUS_INVALID_PARAMETER;
            };
            match EccKey::generate(usage, c) {
                Ok(ec) => {
                    k.data = KeyData::Ecc(ec);
                    STATUS_SUCCESS
                }
                Err(e) => e,
            }
        }
        _ => STATUS_INVALID_HANDLE,
    }
}

pub unsafe extern "win64" fn BCryptSetProperty(
    h_obj: *mut c_void,
    prop: *const u16,
    input: *const u8,
    input_len: u32,
    _flags: u32,
) -> i32 {
    if prop.is_null() || (input_len > 0 && input.is_null()) {
        return STATUS_INVALID_PARAMETER;
    }
    let p = crate::strings::from_wide(prop);
    let in_bytes = bytes(input, input_len);

    if p == "ChainingMode" {
        if in_bytes.len() < 2 || in_bytes.len() % 2 != 0 {
            return STATUS_INVALID_PARAMETER;
        }
        let units: Vec<u16> = in_bytes.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
        let mode_str = String::from_utf16_lossy(&units);
        let Some(mode) = CipherMode::from_name(&mode_str) else {
            log::debug!("BCryptSetProperty(ChainingMode={mode_str:?}) -> STATUS_NOT_SUPPORTED");
            return STATUS_NOT_SUPPORTED;
        };
        if let Some(a) = alg_mut(h_obj) {
            let AlgKind::Symmetric(s) = a.kind else {
                return STATUS_NOT_SUPPORTED;
            };
            if !s.supports_mode(mode) {
                return STATUS_NOT_SUPPORTED;
            }
            a.mode = mode;
            return STATUS_SUCCESS;
        }
        if let Some(k) = key_mut(h_obj) {
            if let KeyData::Symmetric { alg, mode: ref mut km, .. } = k.data {
                if !alg.supports_mode(mode) {
                    return STATUS_NOT_SUPPORTED;
                }
                *km = mode;
                return STATUS_SUCCESS;
            }
            return STATUS_NOT_SUPPORTED;
        }
        return STATUS_INVALID_HANDLE;
    }

    if p == "MessageBlockLength" {
        if in_bytes.len() < 4 {
            return STATUS_INVALID_PARAMETER;
        }
        let mbl = u32::from_le_bytes(in_bytes[0..4].try_into().unwrap());
        if let Some(a) = alg_mut(h_obj) {
            let AlgKind::Symmetric(s) = a.kind else {
                return STATUS_NOT_SUPPORTED;
            };
            let max_bs = s.block_size() as u32;
            if max_bs == 0 || mbl == 0 || mbl > max_bs {
                return STATUS_INVALID_PARAMETER;
            }
            a.msg_block_len = mbl;
            return STATUS_SUCCESS;
        }
        if let Some(k) = key_mut(h_obj) {
            if let KeyData::Symmetric { alg, msg_block_len: ref mut km, .. } = k.data {
                let max_bs = alg.block_size() as u32;
                if max_bs == 0 || mbl == 0 || mbl > max_bs {
                    return STATUS_INVALID_PARAMETER;
                }
                *km = mbl;
                return STATUS_SUCCESS;
            }
            return STATUS_NOT_SUPPORTED;
        }
        return STATUS_INVALID_HANDLE;
    }

    if p == "IV" {
        if let Some(o) = hash_obj(h_obj) {
            if o.kind == HashKind::AesGmac {
                if in_bytes.is_empty() {
                    return STATUS_INVALID_PARAMETER;
                }
                o.iv = in_bytes.to_vec();
                return STATUS_SUCCESS;
            }
            return STATUS_NOT_SUPPORTED;
        }
        if let Some(k) = key_mut(h_obj) {
            if let KeyData::Symmetric { alg, iv: ref mut key_iv, .. } = k.data {
                let bs = alg.block_size();
                if bs == 0 {
                    return STATUS_NOT_SUPPORTED;
                }
                if in_bytes.len() != bs {
                    return STATUS_INVALID_PARAMETER;
                }
                *key_iv = in_bytes.to_vec();
                return STATUS_SUCCESS;
            }
            return STATUS_NOT_SUPPORTED;
        }
        return STATUS_INVALID_HANDLE;
    }

    if p == "EffectiveKeyLength" {
        if in_bytes.len() < 4 {
            return STATUS_INVALID_PARAMETER;
        }
        let bits = u32::from_le_bytes(in_bytes[0..4].try_into().unwrap());
        if !(40..=1024).contains(&bits) {
            return STATUS_INVALID_PARAMETER;
        }
        if let Some(a) = alg_mut(h_obj) {
            if a.kind == AlgKind::Symmetric(SymAlg::Rc2) {
                a.effective_key_len = bits;
                return STATUS_SUCCESS;
            }
            return STATUS_NOT_SUPPORTED;
        }
        if let Some(k) = key_mut(h_obj) {
            if let KeyData::Symmetric { alg: SymAlg::Rc2, effective_key_len: ref mut ek, .. } = k.data {
                *ek = bits;
                return STATUS_SUCCESS;
            }
            return STATUS_NOT_SUPPORTED;
        }
        return STATUS_INVALID_HANDLE;
    }

    if p == "ECCCurveName" {
        if in_bytes.len() < 2 || in_bytes.len() % 2 != 0 {
            return STATUS_INVALID_PARAMETER;
        }
        let units: Vec<u16> = in_bytes.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
        let curve_str = String::from_utf16_lossy(&units);
        let Some(curve) = EccCurve::from_win_curve_name(&curve_str) else {
            log::debug!("BCryptSetProperty(ECCCurveName={curve_str:?}) -> STATUS_NOT_SUPPORTED");
            return STATUS_NOT_SUPPORTED;
        };
        if let Some(a) = alg_mut(h_obj) {
            if matches!(a.kind, AlgKind::Ecc { curve: None, .. }) {
                a.ecc_curve = Some(curve);
                return STATUS_SUCCESS;
            }
            return STATUS_NOT_SUPPORTED;
        }
        if let Some(k) = key_mut(h_obj) {
            if let KeyData::PendingEcc { curve: ref mut kc, .. } = k.data {
                *kc = Some(curve);
                return STATUS_SUCCESS;
            }
            return STATUS_NOT_SUPPORTED;
        }
        return STATUS_INVALID_HANDLE;
    }

    if p == "KeyLength" {
        if in_bytes.len() < 4 {
            return STATUS_INVALID_PARAMETER;
        }
        let bits = u32::from_le_bytes(in_bytes[0..4].try_into().unwrap());
        if let Some(k) = key_mut(h_obj) {
            match k.data {
                KeyData::PendingRsa { bits: ref mut kb } => {
                    if !(1024..=4096).contains(&bits) || bits % 64 != 0 {
                        return STATUS_INVALID_PARAMETER;
                    }
                    *kb = bits;
                    return STATUS_SUCCESS;
                }
                KeyData::PendingEcc { curve: ref mut kc, .. } => {
                    let Some(c) = EccCurve::from_bits(bits) else {
                        return STATUS_INVALID_PARAMETER;
                    };
                    *kc = Some(c);
                    return STATUS_SUCCESS;
                }
                _ => return STATUS_NOT_SUPPORTED,
            }
        }
        return STATUS_INVALID_HANDLE;
    }

    log::debug!("BCryptSetProperty({p:?}) -> STATUS_NOT_SUPPORTED");
    STATUS_NOT_SUPPORTED
}

pub unsafe extern "win64" fn BCryptDuplicateHash(
    h_hash: *mut c_void,
    ph_new: *mut *mut c_void,
    _obj: *mut u8,
    _obj_len: u32,
    _flags: u32,
) -> i32 {
    let Some(o) = hash_obj(h_hash) else { return STATUS_INVALID_HANDLE };
    if ph_new.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let dup = Hash {
        magic: HASH_MAGIC,
        kind: o.kind,
        key: o.key.clone(),
        iv: o.iv.clone(),
        reusable: o.reusable,
        data: o.data.clone(),
    };
    std::ptr::write_unaligned(ph_new, Box::into_raw(Box::new(dup)) as *mut c_void);
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn BCryptSecretAgreement(
    h_priv_key: *mut c_void,
    h_pub_key: *mut c_void,
    ph_agreed_secret: *mut *mut c_void,
    _flags: u32,
) -> i32 {
    if !ph_agreed_secret.is_null() {
        std::ptr::write_unaligned(ph_agreed_secret, std::ptr::null_mut());
    }
    if ph_agreed_secret.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let Some(kp) = key(h_priv_key) else {
        return STATUS_INVALID_HANDLE;
    };
    let Some(kq) = key(h_pub_key) else {
        return STATUS_INVALID_HANDLE;
    };
    let (KeyData::Ecc(priv_ec), KeyData::Ecc(pub_ec)) = (&kp.data, &kq.data) else {
        return STATUS_INVALID_HANDLE;
    };
    if priv_ec.usage != EccUsage::Ecdh || pub_ec.usage != EccUsage::Ecdh {
        return STATUS_INVALID_HANDLE;
    }
    let secret = match priv_ec.ecdh_agree(pub_ec) {
        Ok(s) => s,
        Err(e) => return e,
    };
    std::ptr::write_unaligned(
        ph_agreed_secret,
        Box::into_raw(Box::new(Secret { magic: SECRET_MAGIC, secret })) as *mut c_void,
    );
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn BCryptDestroySecret(h_secret: *mut c_void) -> i32 {
    if secret_obj(h_secret).is_none() {
        return STATUS_INVALID_HANDLE;
    }
    let mut s = Box::from_raw(h_secret as *mut Secret);
    s.magic = 0;
    STATUS_SUCCESS
}

fn p_hash(d: Digest, secret: &[u8], seed: &[u8], out_len: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(out_len + d.len());
    let mut a = d.hmac(secret, seed);
    while out.len() < out_len {
        let mut msg = Vec::with_capacity(a.len() + seed.len());
        msg.extend_from_slice(&a);
        msg.extend_from_slice(seed);
        out.extend_from_slice(&d.hmac(secret, &msg));
        a = d.hmac(secret, &a);
    }
    out.truncate(out_len);
    out
}

pub unsafe extern "win64" fn BCryptDeriveKey(
    h_shared_secret: *mut c_void,
    pwsz_kdf: *const u16,
    p_param_list: *const c_void,
    pb_derived_key: *mut u8,
    cb_derived_key: u32,
    pcb_result: *mut u32,
    flags: u32,
) -> i32 {
    let Some(sec) = secret_obj(h_shared_secret) else {
        return STATUS_INVALID_HANDLE;
    };
    if pwsz_kdf.is_null() || pcb_result.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let kdf = crate::strings::from_wide(pwsz_kdf);

    // Parse optional BCryptBufferDesc parameters.
    let mut hash_alg: Option<Digest> = None;
    let mut prepend: Vec<u8> = Vec::new();
    let mut append: Vec<u8> = Vec::new();
    let mut hmac_key: Option<Vec<u8>> = None;
    let mut tls_label: Vec<u8> = Vec::new();
    let mut tls_seed: Vec<u8> = Vec::new();
    let mut alg_id: Vec<u8> = Vec::new();
    let mut party_u: Vec<u8> = Vec::new();
    let mut party_v: Vec<u8> = Vec::new();
    let mut supp_pub: Vec<u8> = Vec::new();
    let mut supp_priv: Vec<u8> = Vec::new();

    if !p_param_list.is_null() {
        let desc = std::ptr::read_unaligned(p_param_list as *const BcryptBufferDesc);
        if desc.c_buffers > 0 && desc.p_buffers.is_null() {
            return STATUS_INVALID_PARAMETER;
        }
        for i in 0..desc.c_buffers as usize {
            let b = std::ptr::read_unaligned(desc.p_buffers.add(i));
            let slice = bytes(b.pv_buffer as *const u8, b.cb_buffer);
            match b.buffer_type {
                KDF_HASH_ALGORITHM => {
                    if slice.len() >= 2 && slice.len() % 2 == 0 {
                        let units: Vec<u16> = slice.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
                        let name = String::from_utf16_lossy(&units);
                        let clean = name.trim_end_matches('\0');
                        let Some(d) = Digest::from_name(clean) else {
                            return STATUS_NOT_SUPPORTED;
                        };
                        hash_alg = Some(d);
                    }
                }
                KDF_SECRET_PREPEND => prepend = slice.to_vec(),
                KDF_SECRET_APPEND => append = slice.to_vec(),
                KDF_HMAC_KEY => hmac_key = Some(slice.to_vec()),
                KDF_TLS_PRF_LABEL => tls_label = slice.to_vec(),
                KDF_TLS_PRF_SEED => tls_seed = slice.to_vec(),
                KDF_ALGORITHMID => alg_id = slice.to_vec(),
                KDF_PARTYUINFO => party_u = slice.to_vec(),
                KDF_PARTYVINFO => party_v = slice.to_vec(),
                KDF_SUPPPUBINFO => supp_pub = slice.to_vec(),
                KDF_SUPPPRIVINFO => supp_priv = slice.to_vec(),
                _ => {}
            }
        }
    }

    let derived: Vec<u8> = match kdf.as_str() {
        "TRUNCATE" => {
            // Microsoft Learn: BCRYPT_KDF_RAW_SECRET returns the raw secret in little-endian byte order.
            rev_bytes(&sec.secret)
        }
        "HASH" => {
            let d = hash_alg.unwrap_or(Digest::Sha1);
            d.hash_slices(&[&prepend, &sec.secret, &append])
        }
        "HMAC" => {
            let d = hash_alg.unwrap_or(Digest::Sha1);
            let use_secret_as_key = (flags & KDF_USE_SECRET_AS_HMAC_KEY_FLAG) != 0;
            let (k, msg): (&[u8], Vec<u8>) = if use_secret_as_key {
                (&sec.secret, [&prepend[..], &append[..]].concat())
            } else {
                let Some(ref hk) = hmac_key else {
                    return STATUS_INVALID_PARAMETER;
                };
                (hk, [&prepend[..], &sec.secret[..], &append[..]].concat())
            };
            d.hmac(k, &msg)
        }
        "TLS_PRF" => {
            let want = if pb_derived_key.is_null() { 48usize } else { cb_derived_key as usize };
            let mut seed = Vec::with_capacity(tls_label.len() + tls_seed.len());
            seed.extend_from_slice(&tls_label);
            seed.extend_from_slice(&tls_seed);
            if let Some(d) = hash_alg {
                p_hash(d, &sec.secret, &seed, want)
            } else {
                // TLS 1.0/1.1 PRF: P_MD5(S1, seed) XOR P_SHA1(S2, seed)
                let half = sec.secret.len().div_ceil(2);
                let s1 = &sec.secret[..half];
                let s2 = &sec.secret[sec.secret.len() - half..];
                let p1 = p_hash(Digest::Md5, s1, &seed, want);
                let p2 = p_hash(Digest::Sha1, s2, &seed, want);
                p1.iter().zip(&p2).map(|(a, b)| a ^ b).collect()
            }
        }
        "SP800_56A_CONCAT" => {
            let d = hash_alg.unwrap_or(Digest::Sha256);
            let want = if pb_derived_key.is_null() { d.len() } else { cb_derived_key as usize };
            let reps = want.div_ceil(d.len());
            let mut out = Vec::with_capacity(reps * d.len());
            for ctr in 1..=(reps as u32) {
                let cbe = ctr.to_be_bytes();
                let blk = d.hash_slices(&[&cbe, &sec.secret, &alg_id, &party_u, &party_v, &supp_pub, &supp_priv]);
                out.extend_from_slice(&blk);
            }
            out.truncate(want);
            out
        }
        _ => return STATUS_NOT_SUPPORTED,
    };

    if pb_derived_key.is_null() {
        std::ptr::write_unaligned(pcb_result, derived.len() as u32);
        return STATUS_SUCCESS;
    }
    let copy_len = derived.len().min(cb_derived_key as usize);
    if copy_len > 0 {
        std::ptr::copy_nonoverlapping(derived.as_ptr(), pb_derived_key, copy_len);
    }
    std::ptr::write_unaligned(pcb_result, copy_len as u32);
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn BCryptDeriveKeyPBKDF2(
    h_prf: *mut c_void,
    pass: *const u8,
    pass_len: u32,
    salt: *const u8,
    salt_len: u32,
    iters: u64,
    derived: *mut u8,
    derived_len: u32,
    _flags: u32,
) -> i32 {
    let Some(a) = alg(h_prf) else {
        return STATUS_INVALID_HANDLE;
    };
    let AlgKind::Hash(digest) = a.kind else {
        return STATUS_INVALID_HANDLE;
    };
    if iters == 0 || derived.is_null() || derived_len == 0 {
        return STATUS_INVALID_PARAMETER;
    }
    let pass_b = bytes(pass, pass_len);
    let salt_b = bytes(salt, salt_len);
    let hlen = digest.len();
    let dk_len = derived_len as usize;
    let blocks = dk_len.div_ceil(hlen);
    let mut out = Vec::with_capacity(blocks * hlen);
    for blk in 1..=(blocks as u32) {
        let mut init = Vec::with_capacity(salt_b.len() + 4);
        init.extend_from_slice(salt_b);
        init.extend_from_slice(&blk.to_be_bytes());
        let mut u = digest.hmac(pass_b, &init);
        let mut t = u.clone();
        for _ in 1..iters {
            u = digest.hmac(pass_b, &u);
            for (ti, &ui) in t.iter_mut().zip(u.iter()) {
                *ti ^= ui;
            }
        }
        out.extend_from_slice(&t);
    }
    std::ptr::copy_nonoverlapping(out.as_ptr(), derived, dk_len);
    STATUS_SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(s: &str) -> Vec<u8> {
        (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
    }

    // 1024-bit test key and signatures over "aw2 license test" made with
    // `openssl dgst -sha256 -sign` (PKCS#1 v1.5) and `-sigopt rsa_padding_mode:pss
    // -sigopt rsa_pss_saltlen:32`.
    const N: &str = "CAA2D2172A1AE3E2C6EFB357E25B0C80B4CC77044032EA5AA7C8369F0C7F9786751D7FE853A0FA801EBB4CF3FBC861138952CFD50D1DFC8E5BF91E98DB4D72B369B8A0C0D4E1FC67B89F223152C9DF04049633CE988BE3050BCF92A606B0E65A2DD12E951EC0D4D4A3B1F1732255CFD81FD08164780F02A9288F6998F913A7D1";
    const SIG_PKCS1: &str = "c454e628bf702ee02de25cee7d12c50a459b587bbd50e52246fdb5ad8d4c153437fa8283cbf396dd567c1e6d5c741010f08c87d4fbd895b335e1abf882675a6a8089f1d1508583491f858399a1834536a53094208f5d6cb2e0f8a36d51a56b41e6d20a75e84e4e62cfb5020a3b41532997b7abc208275677c31e5fc3aadac2b9";
    const SIG_PSS: &str = "3dd83b73b3d59e6888a42bc2560b0205c0c2a937609b53a5789e26412e58f03c6a82c497bf8f5bf4198e8acc37762383f05f1ad7757270d33bd94fb559848bb3e3b585809a1e6d222f2834809ecfc2146e26ef70137c639c112bb17c7820967245230ad839ba2c8d4798281b6535c498284cbfef3f9887f1ea58f306fbc4434d";

    fn blob() -> Vec<u8> {
        let n = hex(N);
        let e = vec![1u8, 0, 1];
        let mut b = Vec::new();
        for v in [BCRYPT_RSAPUBLIC_MAGIC, 1024, e.len() as u32, n.len() as u32, 0, 0] {
            b.extend(v.to_le_bytes());
        }
        b.extend(e);
        b.extend(n);
        b
    }

    #[test]
    fn sha256_and_hmac() {
        assert_eq!(Digest::Sha256.hash(b"aw2 license test"), hex("2384ba78aa1f8fc0a347ad99ffef6c44cb3d32c46ea52af9510d6d706047ffa2"));
        // RFC 4231 test case 2.
        assert_eq!(
            Digest::Sha256.hmac(b"Jefe", b"what do ya want for nothing?"),
            hex("5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843")
        );
    }

    #[test]
    fn modexp_small() {
        // 4^13 mod 497 = 445
        assert_eq!(bn_modexp(&[4], &[13], &[497]), vec![445]);
        assert_eq!(bn_to_be(&bn_from_be(&[1, 2, 3, 4, 5]), 5), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn rsa_verify_pkcs1_and_pss() {
        let k = RsaPublicKey::from_blob(&blob()).unwrap();
        let h = Digest::Sha256.hash(b"aw2 license test");
        assert!(k.verify_pkcs1(Some(Digest::Sha256), &h, &hex(SIG_PKCS1)));
        assert!(k.verify_pss(Digest::Sha256, &h, 32, &hex(SIG_PSS)));
        let mut bad = h.clone();
        bad[0] ^= 1;
        assert!(!k.verify_pkcs1(Some(Digest::Sha256), &bad, &hex(SIG_PKCS1)));
        assert!(!k.verify_pss(Digest::Sha256, &bad, 32, &hex(SIG_PSS)));
        assert!(!k.verify_pkcs1(Some(Digest::Sha256), &h, &hex(SIG_PSS)));
    }

    #[test]
    fn cng_api_flow() {
        unsafe {
            let w = |s: &str| s.encode_utf16().chain([0]).collect::<Vec<u16>>();
            let mut ha = std::ptr::null_mut();
            assert_eq!(BCryptOpenAlgorithmProvider(&mut ha, w("SHA256").as_ptr(), std::ptr::null(), 0), 0);
            let mut v = 0u32;
            let mut got = 0u32;
            assert_eq!(BCryptGetProperty(ha, w("HashDigestLength").as_ptr(), &mut v as *mut u32 as *mut u8, 4, &mut got, 0), 0);
            assert_eq!((v, got), (32, 4));
            let mut hh = std::ptr::null_mut();
            assert_eq!(BCryptCreateHash(ha, &mut hh, std::ptr::null_mut(), 0, std::ptr::null_mut(), 0, 0), 0);
            let msg = b"aw2 license test";
            assert_eq!(BCryptHashData(hh, msg[..4].as_ptr() as *mut u8, 4, 0), 0);
            assert_eq!(BCryptHashData(hh, msg[4..].as_ptr() as *mut u8, (msg.len() - 4) as u32, 0), 0);
            let mut digest = [0u8; 32];
            assert_eq!(BCryptFinishHash(hh, digest.as_mut_ptr(), 32, 0), 0);
            assert_eq!(BCryptDestroyHash(hh), 0);

            let mut hr = std::ptr::null_mut();
            assert_eq!(BCryptOpenAlgorithmProvider(&mut hr, w("RSA").as_ptr(), std::ptr::null(), 0), 0);
            let mut b = blob();
            let mut hk = std::ptr::null_mut();
            assert_eq!(BCryptImportKeyPair(hr, std::ptr::null_mut(), w("RSAPUBLICBLOB").as_ptr(), &mut hk, b.as_mut_ptr(), b.len() as u32, 0), 0);
            let alg_id = w("SHA256");
            let mut pad: *const u16 = alg_id.as_ptr();
            let mut sig = hex(SIG_PKCS1);
            let r = BCryptVerifySignature(hk, &mut pad as *mut _ as *mut c_void, digest.as_mut_ptr(), 32, sig.as_mut_ptr(), sig.len() as u32, BCRYPT_PAD_PKCS1);
            assert_eq!(r, STATUS_SUCCESS);
            sig[5] ^= 0x40;
            let r = BCryptVerifySignature(hk, &mut pad as *mut _ as *mut c_void, digest.as_mut_ptr(), 32, sig.as_mut_ptr(), sig.len() as u32, BCRYPT_PAD_PKCS1);
            assert_eq!(r, STATUS_INVALID_SIGNATURE);
            assert_eq!(BCryptDestroyKey(hk), 0);
            assert_eq!(BCryptCloseAlgorithmProvider(hr, 0), 0);
            assert_eq!(BCryptCloseAlgorithmProvider(ha, 0), 0);
        }
    }

    #[test]
    fn aes_ecb_cbc_ctr_gcm_and_pbkdf2_nist_vectors() {
        unsafe {
            let w = |s: &str| s.encode_utf16().chain([0]).collect::<Vec<u16>>();
            let wbytes = |s: &str| -> Vec<u8> { w(s).into_iter().flat_map(u16::to_le_bytes).collect() };

            // --- NIST SP 800-38A AES-128-CBC vector ---
            let mut h_alg = std::ptr::null_mut();
            assert_eq!(BCryptOpenAlgorithmProvider(&mut h_alg, w("AES").as_ptr(), std::ptr::null(), 0), STATUS_SUCCESS);
            let mode_cbc = wbytes("ChainingModeCBC");
            assert_eq!(BCryptSetProperty(h_alg, w("ChainingMode").as_ptr(), mode_cbc.as_ptr(), mode_cbc.len() as u32, 0), STATUS_SUCCESS);

            let key128 = hex("2b7e151628aed2a6abf7158809cf4f3c");
            let mut h_key = std::ptr::null_mut();
            assert_eq!(BCryptGenerateSymmetricKey(h_alg, &mut h_key, std::ptr::null_mut(), 0, key128.as_ptr(), 16, 0), STATUS_SUCCESS);

            let pt = hex("6bc1bee22e409f96e93d7e117393172a");
            let expected_ct = hex("7649abac8119b246cee98e9b12e9197d");
            let mut iv = hex("000102030405060708090a0b0c0d0e0f");
            let mut ct = vec![0u8; 16];
            let mut out_len = 0u32;
            assert_eq!(
                BCryptEncrypt(h_key, pt.as_ptr(), 16, std::ptr::null_mut(), iv.as_mut_ptr(), 16, ct.as_mut_ptr(), 16, &mut out_len, 0),
                STATUS_SUCCESS
            );
            assert_eq!(out_len, 16);
            assert_eq!(ct, expected_ct);
            assert_eq!(iv, expected_ct); // CBC updates IV in-place

            let mut iv2 = hex("000102030405060708090a0b0c0d0e0f");
            let mut dec = vec![0u8; 16];
            assert_eq!(
                BCryptDecrypt(h_key, ct.as_ptr(), 16, std::ptr::null_mut(), iv2.as_mut_ptr(), 16, dec.as_mut_ptr(), 16, &mut out_len, 0),
                STATUS_SUCCESS
            );
            assert_eq!(dec, pt);
            assert_eq!(BCryptDestroyKey(h_key), STATUS_SUCCESS);

            // --- NIST SP 800-38D Test Case 4 (AES-128-GCM with AAD) ---
            let mode_gcm = wbytes("ChainingModeGCM");
            assert_eq!(BCryptSetProperty(h_alg, w("ChainingMode").as_ptr(), mode_gcm.as_ptr(), mode_gcm.len() as u32, 0), STATUS_SUCCESS);
            let gcm_key = hex("feffe9928665731c6d6a8f9467308308");
            let gcm_pt = hex("d9313225f88406e5a55909c5aff5269a86a7a9531534f7da2e4c303d8a318a721c3c0c95956809532fcf0e2449a6b525b16aedf5aa0de657ba637b39");
            let gcm_aad = hex("feedfacedeadbeeffeedfacedeadbeefabaddad2");
            let mut gcm_iv = hex("cafebabefacedbaddecaf888");
            let expected_gcm_ct = hex("42831ec2217774244b7221b784d0d49ce3aa212f2c02a4e035c17e2329aca12e21d514b25466931c7d8f6a5aac84aa051ba30b396a0aac973d58e091");
            let expected_gcm_tag = hex("5bc94fbc3221a5db94fae95ae7121a47");

            let mut h_gcm = std::ptr::null_mut();
            assert_eq!(BCryptGenerateSymmetricKey(h_alg, &mut h_gcm, std::ptr::null_mut(), 0, gcm_key.as_ptr(), 16, 0), STATUS_SUCCESS);
            let mut tag = [0u8; 16];
            let mut auth_info = BcryptAuthCipherModeInfo {
                cb_size: std::mem::size_of::<BcryptAuthCipherModeInfo>() as u32,
                dw_info_version: 1,
                pb_nonce: gcm_iv.as_mut_ptr(),
                cb_nonce: gcm_iv.len() as u32,
                pb_auth_data: gcm_aad.as_ptr() as *mut u8,
                cb_auth_data: gcm_aad.len() as u32,
                pb_tag: tag.as_mut_ptr(),
                cb_tag: 16,
                pb_mac_context: std::ptr::null_mut(),
                cb_mac_context: 0,
                cb_aad: 0,
                cb_data: 0,
                dw_flags: 0,
            };
            let mut gcm_ct = vec![0u8; gcm_pt.len()];
            assert_eq!(
                BCryptEncrypt(
                    h_gcm,
                    gcm_pt.as_ptr(),
                    gcm_pt.len() as u32,
                    &mut auth_info as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                    0,
                    gcm_ct.as_mut_ptr(),
                    gcm_ct.len() as u32,
                    &mut out_len,
                    0,
                ),
                STATUS_SUCCESS
            );
            assert_eq!(gcm_ct, expected_gcm_ct);
            assert_eq!(tag.as_slice(), expected_gcm_tag.as_slice());

            let mut gcm_dec = vec![0u8; gcm_ct.len()];
            assert_eq!(
                BCryptDecrypt(
                    h_gcm,
                    gcm_ct.as_ptr(),
                    gcm_ct.len() as u32,
                    &mut auth_info as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                    0,
                    gcm_dec.as_mut_ptr(),
                    gcm_dec.len() as u32,
                    &mut out_len,
                    0,
                ),
                STATUS_SUCCESS
            );
            assert_eq!(gcm_dec, gcm_pt);

            // Tampered tag MUST fail with STATUS_AUTH_TAG_MISMATCH
            let mut bad_tag = tag;
            bad_tag[0] ^= 0x80;
            auth_info.pb_tag = bad_tag.as_mut_ptr();
            assert_eq!(
                BCryptDecrypt(
                    h_gcm,
                    gcm_ct.as_ptr(),
                    gcm_ct.len() as u32,
                    &mut auth_info as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                    0,
                    gcm_dec.as_mut_ptr(),
                    gcm_dec.len() as u32,
                    &mut out_len,
                    0,
                ),
                STATUS_AUTH_TAG_MISMATCH
            );
            assert_eq!(BCryptDestroyKey(h_gcm), STATUS_SUCCESS);
            assert_eq!(BCryptCloseAlgorithmProvider(h_alg, 0), STATUS_SUCCESS);

            // --- RFC 6070 PBKDF2-HMAC-SHA1 test vector (iters=2, dkLen=20) ---
            let mut h_sha1 = std::ptr::null_mut();
            assert_eq!(
                BCryptOpenAlgorithmProvider(&mut h_sha1, w("SHA1").as_ptr(), std::ptr::null(), BCRYPT_ALG_HANDLE_HMAC_FLAG),
                STATUS_SUCCESS
            );
            let mut dk = [0u8; 20];
            assert_eq!(
                BCryptDeriveKeyPBKDF2(h_sha1, b"password".as_ptr(), 8, b"salt".as_ptr(), 4, 2, dk.as_mut_ptr(), 20, 0),
                STATUS_SUCCESS
            );
            assert_eq!(dk.as_slice(), hex("ea6c014dc72d6f8ccd1ed92ace1d41f0d8de8957").as_slice());
            assert_eq!(BCryptCloseAlgorithmProvider(h_sha1, 0), STATUS_SUCCESS);
        }
    }

    #[test]
    fn aes_ecb_cfb_ctr_and_key_import_export_roundtrip() {
        unsafe {
            let w = |s: &str| s.encode_utf16().chain([0]).collect::<Vec<u16>>();
            let wbytes = |s: &str| -> Vec<u8> { w(s).into_iter().flat_map(u16::to_le_bytes).collect() };

            let mut h_alg = std::ptr::null_mut();
            assert_eq!(BCryptOpenAlgorithmProvider(&mut h_alg, w("AES").as_ptr(), std::ptr::null(), 0), STATUS_SUCCESS);

            // Verify standard algorithm property queries
            let mut blk_len = 0u32;
            let mut got = 0u32;
            assert_eq!(BCryptGetProperty(h_alg, w("BlockLength").as_ptr(), &mut blk_len as *mut u32 as *mut u8, 4, &mut got, 0), STATUS_SUCCESS);
            assert_eq!((blk_len, got), (16, 4));
            let mut key_lens = [0u32; 3];
            assert_eq!(BCryptGetProperty(h_alg, w("KeyLengths").as_ptr(), key_lens.as_mut_ptr() as *mut u8, 12, &mut got, 0), STATUS_SUCCESS);
            assert_eq!((key_lens, got), ([128, 256, 64], 12));
            let mut tag_lens = [0u32; 3];
            assert_eq!(BCryptGetProperty(h_alg, w("AuthTagLength").as_ptr(), tag_lens.as_mut_ptr() as *mut u8, 12, &mut got, 0), STATUS_SUCCESS);
            assert_eq!((tag_lens, got), ([12, 16, 1], 12));

            let key128 = hex("2b7e151628aed2a6abf7158809cf4f3c");
            let pt32 = hex("6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e51");

            // --- NIST SP 800-38A AES-128-ECB ---
            let mode_ecb = wbytes("ChainingModeECB");
            assert_eq!(BCryptSetProperty(h_alg, w("ChainingMode").as_ptr(), mode_ecb.as_ptr(), mode_ecb.len() as u32, 0), STATUS_SUCCESS);
            let mut h_ecb = std::ptr::null_mut();
            assert_eq!(BCryptGenerateSymmetricKey(h_alg, &mut h_ecb, std::ptr::null_mut(), 0, key128.as_ptr(), 16, 0), STATUS_SUCCESS);
            let mut ct_ecb = vec![0u8; 32];
            let mut out_len = 0u32;
            assert_eq!(
                BCryptEncrypt(h_ecb, pt32.as_ptr(), 32, std::ptr::null_mut(), std::ptr::null_mut(), 0, ct_ecb.as_mut_ptr(), 32, &mut out_len, 0),
                STATUS_SUCCESS
            );
            assert_eq!(ct_ecb, hex("3ad77bb40d7a3660a89ecaf32466ef97f5d3d58503b9699de785895a96fdbaaf"));
            assert_eq!(BCryptDestroyKey(h_ecb), STATUS_SUCCESS);

            // --- NIST SP 800-38A AES-128-CFB128 (F.3.13) ---
            let mode_cfb = wbytes("ChainingModeCFB");
            assert_eq!(BCryptSetProperty(h_alg, w("ChainingMode").as_ptr(), mode_cfb.as_ptr(), mode_cfb.len() as u32, 0), STATUS_SUCCESS);
            let mbl16 = 16u32.to_le_bytes();
            assert_eq!(BCryptSetProperty(h_alg, w("MessageBlockLength").as_ptr(), mbl16.as_ptr(), 4, 0), STATUS_SUCCESS);
            let mut h_cfb = std::ptr::null_mut();
            assert_eq!(BCryptGenerateSymmetricKey(h_alg, &mut h_cfb, std::ptr::null_mut(), 0, key128.as_ptr(), 16, 0), STATUS_SUCCESS);

            let mut iv_cfb = hex("000102030405060708090a0b0c0d0e0f");
            let mut ct_cfb = vec![0u8; 32];
            assert_eq!(
                BCryptEncrypt(h_cfb, pt32.as_ptr(), 32, std::ptr::null_mut(), iv_cfb.as_mut_ptr(), 16, ct_cfb.as_mut_ptr(), 32, &mut out_len, 0),
                STATUS_SUCCESS
            );
            assert_eq!(out_len, 32);
            assert_eq!(ct_cfb, hex("3b3fd92eb72dad20333449f8e83cfb4ac8a64537a0b3a93fcde3cdad9f1ce58b"));

            // Export as OpaqueKeyBlob (preserves CFB mode + msg_block_len) & re-import to decrypt!
            let mut opq_len = 0u32;
            assert_eq!(BCryptExportKey(h_cfb, std::ptr::null_mut(), w("OpaqueKeyBlob").as_ptr(), std::ptr::null_mut(), 0, &mut opq_len, 0), STATUS_SUCCESS);
            let mut opq_blob = vec![0u8; opq_len as usize];
            assert_eq!(BCryptExportKey(h_cfb, std::ptr::null_mut(), w("OpaqueKeyBlob").as_ptr(), opq_blob.as_mut_ptr(), opq_len, &mut opq_len, 0), STATUS_SUCCESS);

            // Switch alg mode away to verify OpaqueKeyBlob restores CFB!
            assert_eq!(BCryptSetProperty(h_alg, w("ChainingMode").as_ptr(), mode_ecb.as_ptr(), mode_ecb.len() as u32, 0), STATUS_SUCCESS);
            let mut h_cfb_imp = std::ptr::null_mut();
            assert_eq!(
                BCryptImportKey(h_alg, std::ptr::null_mut(), w("OpaqueKeyBlob").as_ptr(), &mut h_cfb_imp, std::ptr::null_mut(), 0, opq_blob.as_ptr(), opq_len, 0),
                STATUS_SUCCESS
            );
            let mut iv_cfb2 = hex("000102030405060708090a0b0c0d0e0f");
            let mut dec_cfb = vec![0u8; 32];
            assert_eq!(
                BCryptDecrypt(h_cfb_imp, ct_cfb.as_ptr(), 32, std::ptr::null_mut(), iv_cfb2.as_mut_ptr(), 16, dec_cfb.as_mut_ptr(), 32, &mut out_len, 0),
                STATUS_SUCCESS
            );
            assert_eq!(dec_cfb, pt32);

            // --- NIST SP 800-38A AES-128-CFB8 (F.3.7) ---
            let mbl1 = 1u32.to_le_bytes();
            assert_eq!(BCryptSetProperty(h_cfb_imp, w("MessageBlockLength").as_ptr(), mbl1.as_ptr(), 4, 0), STATUS_SUCCESS);
            let mut iv_cfb8 = hex("000102030405060708090a0b0c0d0e0f");
            let pt_cfb8 = hex("6bc1bee22e409f96e93d7e117393172aae2d");
            let mut ct_cfb8 = vec![0u8; pt_cfb8.len()];
            assert_eq!(
                BCryptEncrypt(h_cfb_imp, pt_cfb8.as_ptr(), pt_cfb8.len() as u32, std::ptr::null_mut(), iv_cfb8.as_mut_ptr(), 16, ct_cfb8.as_mut_ptr(), ct_cfb8.len() as u32, &mut out_len, 0),
                STATUS_SUCCESS
            );
            assert_eq!(ct_cfb8, hex("3b79424c9c0dd436bace9e0ed4586a4f32b9"));
            let mut iv_cfb8_d = hex("000102030405060708090a0b0c0d0e0f");
            let mut dec_cfb8 = vec![0u8; ct_cfb8.len()];
            assert_eq!(
                BCryptDecrypt(h_cfb_imp, ct_cfb8.as_ptr(), ct_cfb8.len() as u32, std::ptr::null_mut(), iv_cfb8_d.as_mut_ptr(), 16, dec_cfb8.as_mut_ptr(), dec_cfb8.len() as u32, &mut out_len, 0),
                STATUS_SUCCESS
            );
            assert_eq!(dec_cfb8, pt_cfb8);
            assert_eq!(BCryptDestroyKey(h_cfb_imp), STATUS_SUCCESS);

            // --- KeyDataBlob export & import + NIST SP 800-38A AES-128-CTR (F.5.1) ---
            let mut kdb_len = 0u32;
            assert_eq!(BCryptExportKey(h_cfb, std::ptr::null_mut(), w("KeyDataBlob").as_ptr(), std::ptr::null_mut(), 0, &mut kdb_len, 0), STATUS_SUCCESS);
            let mut kdb_blob = vec![0u8; kdb_len as usize];
            assert_eq!(BCryptExportKey(h_cfb, std::ptr::null_mut(), w("KeyDataBlob").as_ptr(), kdb_blob.as_mut_ptr(), kdb_len, &mut kdb_len, 0), STATUS_SUCCESS);
            assert_eq!(BCryptDestroyKey(h_cfb), STATUS_SUCCESS);

            let mode_ctr = wbytes("ChainingModeCTR");
            assert_eq!(BCryptSetProperty(h_alg, w("ChainingMode").as_ptr(), mode_ctr.as_ptr(), mode_ctr.len() as u32, 0), STATUS_SUCCESS);
            let mut h_ctr = std::ptr::null_mut();
            assert_eq!(
                BCryptImportKey(h_alg, std::ptr::null_mut(), w("KeyDataBlob").as_ptr(), &mut h_ctr, std::ptr::null_mut(), 0, kdb_blob.as_ptr(), kdb_len, 0),
                STATUS_SUCCESS
            );
            let mut ctr_iv = hex("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
            let mut ct_ctr = vec![0u8; 32];
            assert_eq!(
                BCryptEncrypt(h_ctr, pt32.as_ptr(), 32, std::ptr::null_mut(), ctr_iv.as_mut_ptr(), 16, ct_ctr.as_mut_ptr(), 32, &mut out_len, 0),
                STATUS_SUCCESS
            );
            assert_eq!(ct_ctr, hex("874d6191b620e3261bef6864990db6ce9806f66b7970fdff8617187bb9fffdff"));
            assert_eq!(ctr_iv, hex("f0f1f2f3f4f5f6f7f8f9fafbfcfdff01")); // incremented twice

            let mut ctr_iv2 = hex("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
            let mut dec_ctr = vec![0u8; 32];
            assert_eq!(
                BCryptDecrypt(h_ctr, ct_ctr.as_ptr(), 32, std::ptr::null_mut(), ctr_iv2.as_mut_ptr(), 16, dec_ctr.as_mut_ptr(), 32, &mut out_len, 0),
                STATUS_SUCCESS
            );
            assert_eq!(dec_ctr, pt32);

            // Verify ChainingMode property on key
            let mut cm_buf = vec![0u8; 64];
            assert_eq!(BCryptGetProperty(h_ctr, w("ChainingMode").as_ptr(), cm_buf.as_mut_ptr(), cm_buf.len() as u32, &mut got, 0), STATUS_SUCCESS);
            assert_eq!(crate::strings::from_wide(cm_buf.as_ptr() as *const u16), "ChainingModeCTR");

            assert_eq!(BCryptDestroyKey(h_ctr), STATUS_SUCCESS);
            assert_eq!(BCryptCloseAlgorithmProvider(h_alg, 0), STATUS_SUCCESS);
        }
    }

    #[test]
    fn ecdsa_p256_p384_keygen_export_import_sign_and_verify() {
        unsafe {
            let w = |s: &str| s.encode_utf16().chain([0]).collect::<Vec<u16>>();
            let wbytes = |s: &str| -> Vec<u8> { w(s).into_iter().flat_map(u16::to_le_bytes).collect() };

            for (alg_name, bits, sig_expected_len, digest) in [
                ("ECDSA_P256", 256u32, 64u32, Digest::Sha256),
                ("ECDSA_P384", 384u32, 96u32, Digest::Sha384),
            ] {
                let mut h_alg = std::ptr::null_mut();
                assert_eq!(BCryptOpenAlgorithmProvider(&mut h_alg, w(alg_name).as_ptr(), std::ptr::null(), 0), STATUS_SUCCESS);

                let mut h_key = std::ptr::null_mut();
                assert_eq!(BCryptGenerateKeyPair(h_alg, &mut h_key, bits, 0), STATUS_SUCCESS);
                // Unfinalized key must reject export/sign
                let mut dummy_len = 0u32;
                assert_eq!(BCryptExportKey(h_key, std::ptr::null_mut(), w("ECCPUBLICBLOB").as_ptr(), std::ptr::null_mut(), 0, &mut dummy_len, 0), STATUS_INVALID_HANDLE);
                assert_eq!(BCryptFinalizeKeyPair(h_key, 0), STATUS_SUCCESS);

                // Query SignatureLength & KeyLength
                let mut slen = 0u32;
                let mut got = 0u32;
                assert_eq!(BCryptGetProperty(h_key, w("SignatureLength").as_ptr(), &mut slen as *mut u32 as *mut u8, 4, &mut got, 0), STATUS_SUCCESS);
                assert_eq!(slen, sig_expected_len);

                // Sign a hash (size query + real sign)
                let mut h = digest.hash(b"cng ecdsa test payload");
                let mut qlen = 0u32;
                assert_eq!(BCryptSignHash(h_key, std::ptr::null_mut(), h.as_ptr(), h.len() as u32, std::ptr::null_mut(), 0, &mut qlen, 0), STATUS_SUCCESS);
                assert_eq!(qlen, sig_expected_len);
                let mut sig = vec![0u8; qlen as usize];
                assert_eq!(BCryptSignHash(h_key, std::ptr::null_mut(), h.as_ptr(), h.len() as u32, sig.as_mut_ptr(), qlen, &mut qlen, 0), STATUS_SUCCESS);
                assert_eq!(qlen, sig_expected_len);

                // Verify with original keypair
                assert_eq!(BCryptVerifySignature(h_key, std::ptr::null_mut(), h.as_mut_ptr(), h.len() as u32, sig.as_mut_ptr(), sig.len() as u32, 0), STATUS_SUCCESS);

                // Export ECCPUBLICBLOB & re-import as public-only key
                let mut pub_len = 0u32;
                assert_eq!(BCryptExportKey(h_key, std::ptr::null_mut(), w("ECCPUBLICBLOB").as_ptr(), std::ptr::null_mut(), 0, &mut pub_len, 0), STATUS_SUCCESS);
                let mut pub_blob = vec![0u8; pub_len as usize];
                assert_eq!(BCryptExportKey(h_key, std::ptr::null_mut(), w("ECCPUBLICBLOB").as_ptr(), pub_blob.as_mut_ptr(), pub_len, &mut pub_len, 0), STATUS_SUCCESS);

                let mut h_pub = std::ptr::null_mut();
                assert_eq!(
                    BCryptImportKeyPair(h_alg, std::ptr::null_mut(), w("ECCPUBLICBLOB").as_ptr(), &mut h_pub, pub_blob.as_mut_ptr(), pub_len, 0),
                    STATUS_SUCCESS
                );
                assert_eq!(BCryptVerifySignature(h_pub, std::ptr::null_mut(), h.as_mut_ptr(), h.len() as u32, sig.as_mut_ptr(), sig.len() as u32, 0), STATUS_SUCCESS);

                // Tampered signature and tampered hash must fail honestly
                let mut bad_sig = sig.clone();
                bad_sig[0] ^= 0x01;
                assert_eq!(BCryptVerifySignature(h_pub, std::ptr::null_mut(), h.as_mut_ptr(), h.len() as u32, bad_sig.as_mut_ptr(), bad_sig.len() as u32, 0), STATUS_INVALID_SIGNATURE);
                let mut bad_h = h.clone();
                bad_h[0] ^= 0xFF;
                assert_eq!(BCryptVerifySignature(h_pub, std::ptr::null_mut(), bad_h.as_mut_ptr(), bad_h.len() as u32, sig.as_mut_ptr(), sig.len() as u32, 0), STATUS_INVALID_SIGNATURE);

                // Export ECCPRIVATEBLOB & re-import to sign again
                let mut priv_len = 0u32;
                assert_eq!(BCryptExportKey(h_key, std::ptr::null_mut(), w("ECCPRIVATEBLOB").as_ptr(), std::ptr::null_mut(), 0, &mut priv_len, 0), STATUS_SUCCESS);
                let mut priv_blob = vec![0u8; priv_len as usize];
                assert_eq!(BCryptExportKey(h_key, std::ptr::null_mut(), w("ECCPRIVATEBLOB").as_ptr(), priv_blob.as_mut_ptr(), priv_len, &mut priv_len, 0), STATUS_SUCCESS);
                let mut h_priv2 = std::ptr::null_mut();
                assert_eq!(
                    BCryptImportKeyPair(h_alg, std::ptr::null_mut(), w("ECCPRIVATEBLOB").as_ptr(), &mut h_priv2, priv_blob.as_mut_ptr(), priv_len, 0),
                    STATUS_SUCCESS
                );
                let mut sig2 = vec![0u8; sig_expected_len as usize];
                assert_eq!(BCryptSignHash(h_priv2, std::ptr::null_mut(), h.as_ptr(), h.len() as u32, sig2.as_mut_ptr(), sig2.len() as u32, &mut qlen, 0), STATUS_SUCCESS);
                assert_eq!(BCryptVerifySignature(h_pub, std::ptr::null_mut(), h.as_mut_ptr(), h.len() as u32, sig2.as_mut_ptr(), sig2.len() as u32, 0), STATUS_SUCCESS);

                assert_eq!(BCryptDestroyKey(h_priv2), STATUS_SUCCESS);
                assert_eq!(BCryptDestroyKey(h_pub), STATUS_SUCCESS);
                assert_eq!(BCryptDestroyKey(h_key), STATUS_SUCCESS);
                assert_eq!(BCryptCloseAlgorithmProvider(h_alg, 0), STATUS_SUCCESS);
            }

            // Generic "ECDSA" provider + BCryptSetProperty("ECCCurveName", "nistP256")
            let mut h_gen = std::ptr::null_mut();
            assert_eq!(BCryptOpenAlgorithmProvider(&mut h_gen, w("ECDSA").as_ptr(), std::ptr::null(), 0), STATUS_SUCCESS);
            let cname = wbytes("nistP256");
            assert_eq!(BCryptSetProperty(h_gen, w("ECCCurveName").as_ptr(), cname.as_ptr(), cname.len() as u32, 0), STATUS_SUCCESS);
            let mut hk = std::ptr::null_mut();
            assert_eq!(BCryptGenerateKeyPair(h_gen, &mut hk, 0, 0), STATUS_SUCCESS);
            assert_eq!(BCryptFinalizeKeyPair(hk, 0), STATUS_SUCCESS);
            let mut h = Digest::Sha256.hash(b"generic ecdsa");
            let mut sig = vec![0u8; 64];
            let mut got = 0u32;
            assert_eq!(BCryptSignHash(hk, std::ptr::null_mut(), h.as_ptr(), 32, sig.as_mut_ptr(), 64, &mut got, 0), STATUS_SUCCESS);
            assert_eq!(BCryptVerifySignature(hk, std::ptr::null_mut(), h.as_mut_ptr(), 32, sig.as_mut_ptr(), 64, 0), STATUS_SUCCESS);
            assert_eq!(BCryptDestroyKey(hk), STATUS_SUCCESS);
            assert_eq!(BCryptCloseAlgorithmProvider(h_gen, 0), STATUS_SUCCESS);
        }
    }

    #[test]
    fn rsa_2048_keygen_export_import_pkcs1_and_pss_sign_verify() {
        unsafe {
            let w = |s: &str| s.encode_utf16().chain([0]).collect::<Vec<u16>>();
            let mut h_rsa = std::ptr::null_mut();
            assert_eq!(BCryptOpenAlgorithmProvider(&mut h_rsa, w("RSA").as_ptr(), std::ptr::null(), 0), STATUS_SUCCESS);

            let mut h_key = std::ptr::null_mut();
            assert_eq!(BCryptGenerateKeyPair(h_rsa, &mut h_key, 2048, 0), STATUS_SUCCESS);
            assert_eq!(BCryptFinalizeKeyPair(h_key, 0), STATUS_SUCCESS);

            // Check KeyLength / BlockLength / SignatureLength properties
            let mut val = 0u32;
            let mut got = 0u32;
            assert_eq!(BCryptGetProperty(h_key, w("KeyLength").as_ptr(), &mut val as *mut u32 as *mut u8, 4, &mut got, 0), STATUS_SUCCESS);
            assert_eq!(val, 2048);
            assert_eq!(BCryptGetProperty(h_key, w("SignatureLength").as_ptr(), &mut val as *mut u32 as *mut u8, 4, &mut got, 0), STATUS_SUCCESS);
            assert_eq!(val, 256);

            // Export RSAPUBLICBLOB and RSAPRIVATEBLOB and re-import both
            let mut pub_len = 0u32;
            assert_eq!(BCryptExportKey(h_key, std::ptr::null_mut(), w("RSAPUBLICBLOB").as_ptr(), std::ptr::null_mut(), 0, &mut pub_len, 0), STATUS_SUCCESS);
            let mut pub_blob = vec![0u8; pub_len as usize];
            assert_eq!(BCryptExportKey(h_key, std::ptr::null_mut(), w("RSAPUBLICBLOB").as_ptr(), pub_blob.as_mut_ptr(), pub_len, &mut pub_len, 0), STATUS_SUCCESS);

            let mut priv_len = 0u32;
            assert_eq!(BCryptExportKey(h_key, std::ptr::null_mut(), w("RSAPRIVATEBLOB").as_ptr(), std::ptr::null_mut(), 0, &mut priv_len, 0), STATUS_SUCCESS);
            let mut priv_blob = vec![0u8; priv_len as usize];
            assert_eq!(BCryptExportKey(h_key, std::ptr::null_mut(), w("RSAPRIVATEBLOB").as_ptr(), priv_blob.as_mut_ptr(), priv_len, &mut priv_len, 0), STATUS_SUCCESS);

            let mut h_pub = std::ptr::null_mut();
            assert_eq!(BCryptImportKeyPair(h_rsa, std::ptr::null_mut(), w("RSAPUBLICBLOB").as_ptr(), &mut h_pub, pub_blob.as_mut_ptr(), pub_len, 0), STATUS_SUCCESS);
            let mut h_priv = std::ptr::null_mut();
            assert_eq!(BCryptImportKeyPair(h_rsa, std::ptr::null_mut(), w("RSAPRIVATEBLOB").as_ptr(), &mut h_priv, priv_blob.as_mut_ptr(), priv_len, 0), STATUS_SUCCESS);

            // PKCS#1 v1.5 sign (via re-imported RSAPRIVATEBLOB!) & verify (via re-imported RSAPUBLICBLOB!)
            let mut h = Digest::Sha256.hash(b"rsa pkcs1 and pss roundtrip");
            let alg_w = w("SHA256");
            let mut pkcs1_pad: *const u16 = alg_w.as_ptr();
            let mut sig = vec![0u8; 256];
            let mut slen = 0u32;
            assert_eq!(
                BCryptSignHash(h_priv, &mut pkcs1_pad as *mut _ as *mut c_void, h.as_ptr(), 32, sig.as_mut_ptr(), 256, &mut slen, BCRYPT_PAD_PKCS1),
                STATUS_SUCCESS
            );
            assert_eq!(slen, 256);
            assert_eq!(
                BCryptVerifySignature(h_pub, &mut pkcs1_pad as *mut _ as *mut c_void, h.as_mut_ptr(), 32, sig.as_mut_ptr(), 256, BCRYPT_PAD_PKCS1),
                STATUS_SUCCESS
            );
            sig[10] ^= 0x55;
            assert_eq!(
                BCryptVerifySignature(h_pub, &mut pkcs1_pad as *mut _ as *mut c_void, h.as_mut_ptr(), 32, sig.as_mut_ptr(), 256, BCRYPT_PAD_PKCS1),
                STATUS_INVALID_SIGNATURE
            );

            // RSA-PSS sign & verify (with cbSalt = 32)
            #[repr(C)]
            struct PssPad {
                alg_id: *const u16,
                cb_salt: u32,
            }
            let mut pss_pad = PssPad { alg_id: alg_w.as_ptr(), cb_salt: 32 };
            let mut pss_sig = vec![0u8; 256];
            assert_eq!(
                BCryptSignHash(h_key, &mut pss_pad as *mut _ as *mut c_void, h.as_ptr(), 32, pss_sig.as_mut_ptr(), 256, &mut slen, BCRYPT_PAD_PSS),
                STATUS_SUCCESS
            );
            assert_eq!(
                BCryptVerifySignature(h_pub, &mut pss_pad as *mut _ as *mut c_void, h.as_mut_ptr(), 32, pss_sig.as_mut_ptr(), 256, BCRYPT_PAD_PSS),
                STATUS_SUCCESS
            );
            // Wrong salt length must fail verification
            let mut wrong_salt = PssPad { alg_id: alg_w.as_ptr(), cb_salt: 20 };
            assert_eq!(
                BCryptVerifySignature(h_pub, &mut wrong_salt as *mut _ as *mut c_void, h.as_mut_ptr(), 32, pss_sig.as_mut_ptr(), 256, BCRYPT_PAD_PSS),
                STATUS_INVALID_SIGNATURE
            );

            assert_eq!(BCryptDestroyKey(h_priv), STATUS_SUCCESS);
            assert_eq!(BCryptDestroyKey(h_pub), STATUS_SUCCESS);
            assert_eq!(BCryptDestroyKey(h_key), STATUS_SUCCESS);
            assert_eq!(BCryptCloseAlgorithmProvider(h_rsa, 0), STATUS_SUCCESS);
        }
    }

    #[test]
    fn process_prng_entropy_and_errors() {
        unsafe {
            assert_eq!(ProcessPrng(std::ptr::null_mut(), 0), crate::types::TRUE);
            assert_eq!(ProcessPrng(std::ptr::null_mut(), 16), crate::types::FALSE);
            assert_eq!(crate::teb::last_error(), crate::types::err::ERROR_INVALID_PARAMETER);

            let mut a = [0u8; 32];
            let mut b = [0u8; 32];
            crate::det_entropy::reset(0);
            assert_eq!(ProcessPrng(a.as_mut_ptr(), a.len()), crate::types::TRUE);
            assert_eq!(ProcessPrng(b.as_mut_ptr(), b.len()), crate::types::TRUE);
            assert_ne!(a, [0u8; 32]);
            assert_ne!(a, b);

            // Deterministic entropy hook must be respected identically to BCryptGenRandom.
            let mut d1 = [0u8; 24];
            let mut d2 = [0u8; 24];
            crate::det_entropy::reset(0x1234_5678);
            assert_eq!(ProcessPrng(d1.as_mut_ptr(), d1.len()), crate::types::TRUE);
            crate::det_entropy::reset(0x1234_5678);
            assert_eq!(BCryptGenRandom(std::ptr::null_mut(), d2.as_mut_ptr(), d2.len() as u32, 0), STATUS_SUCCESS);
            crate::det_entropy::reset(0);
            assert_eq!(d1, d2);
        }
    }

    #[test]
    fn md2_md4_reusable_and_aes_cmac_gmac_vectors() {
        unsafe {
            let w = |s: &str| s.encode_utf16().chain([0]).collect::<Vec<u16>>();
            // RFC 1319 MD2
            assert_eq!(Digest::Md2.hash(b""), hex("8350e5a3e24c153df2275c9f80692773"));
            assert_eq!(Digest::Md2.hash(b"abc"), hex("da853b0d3f88d99b30283a69e6ded6bb"));
            // RFC 1320 MD4
            assert_eq!(Digest::Md4.hash(b""), hex("31d6cfe0d16ae931b73c59d7e0c089c0"));
            assert_eq!(Digest::Md4.hash(b"abc"), hex("a448017aaf21d8525fc10ae87aa6729d"));

            // Reusable MD4 HMAC via CNG API + IsKeyedHash / IsReusableHash properties
            let mut h_md4 = std::ptr::null_mut();
            assert_eq!(
                BCryptOpenAlgorithmProvider(
                    &mut h_md4,
                    w("MD4").as_ptr(),
                    std::ptr::null(),
                    BCRYPT_ALG_HANDLE_HMAC_FLAG | BCRYPT_HASH_REUSABLE_FLAG,
                ),
                STATUS_SUCCESS
            );
            let mut v = 0u32;
            let mut got = 0u32;
            assert_eq!(BCryptGetProperty(h_md4, w("IsKeyedHash").as_ptr(), &mut v as *mut u32 as *mut u8, 4, &mut got, 0), STATUS_SUCCESS);
            assert_eq!(v, 1);
            assert_eq!(BCryptGetProperty(h_md4, w("IsReusableHash").as_ptr(), &mut v as *mut u32 as *mut u8, 4, &mut got, 0), STATUS_SUCCESS);
            assert_eq!(v, 1);

            let key = b"secret-key";
            let mut hh = std::ptr::null_mut();
            assert_eq!(BCryptCreateHash(h_md4, &mut hh, std::ptr::null_mut(), 0, key.as_ptr() as *mut u8, key.len() as u32, 0), STATUS_SUCCESS);
            let mut mac1 = [0u8; 16];
            let mut mac2 = [0u8; 16];
            assert_eq!(BCryptHashData(hh, b"first".as_ptr() as *mut u8, 5, 0), STATUS_SUCCESS);
            assert_eq!(BCryptFinishHash(hh, mac1.as_mut_ptr(), 16, 0), STATUS_SUCCESS);
            assert_eq!(BCryptHashData(hh, b"second".as_ptr() as *mut u8, 6, 0), STATUS_SUCCESS);
            assert_eq!(BCryptFinishHash(hh, mac2.as_mut_ptr(), 16, 0), STATUS_SUCCESS);
            assert_eq!(mac1.as_slice(), Digest::Md4.hmac(key, b"first").as_slice());
            assert_eq!(mac2.as_slice(), Digest::Md4.hmac(key, b"second").as_slice());
            assert_ne!(mac1, mac2);
            assert_eq!(BCryptDestroyHash(hh), STATUS_SUCCESS);
            assert_eq!(BCryptCloseAlgorithmProvider(h_md4, 0), STATUS_SUCCESS);

            // RFC 4493 Section 4 AES-128-CMAC known-answer vectors
            let cmac_key = hex("2b7e151628aed2a6abf7158809cf4f3c");
            let mut h_cmac_alg = std::ptr::null_mut();
            assert_eq!(BCryptOpenAlgorithmProvider(&mut h_cmac_alg, w("AES-CMAC").as_ptr(), std::ptr::null(), 0), STATUS_SUCCESS);
            let cases: &[(&str, &str)] = &[
                ("", "bb1d6929e95937287fa37d129b756746"),
                ("6bc1bee22e409f96e93d7e117393172a", "070a16b46b4d4144f79bdd9dd04a287c"),
                (
                    "6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e5130c81c46a35ce411",
                    "dfa66747de9ae63030ca32611497c827",
                ),
                (
                    "6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e5130c81c46a35ce411e5fbc1191a0a52eff69f2445df4f9b17ad2b417be66c3710",
                    "51f0bebf7e3b9d92fc49741779363cfe",
                ),
            ];
            for (msg_hex, expect_hex) in cases {
                let msg = hex(msg_hex);
                let mut h_cmac = std::ptr::null_mut();
                assert_eq!(
                    BCryptCreateHash(h_cmac_alg, &mut h_cmac, std::ptr::null_mut(), 0, cmac_key.as_ptr() as *mut u8, 16, 0),
                    STATUS_SUCCESS
                );
                if !msg.is_empty() {
                    assert_eq!(BCryptHashData(h_cmac, msg.as_ptr() as *mut u8, msg.len() as u32, 0), STATUS_SUCCESS);
                }
                let mut tag = [0u8; 16];
                assert_eq!(BCryptFinishHash(h_cmac, tag.as_mut_ptr(), 16, 0), STATUS_SUCCESS);
                assert_eq!(tag.as_slice(), hex(expect_hex).as_slice());
                assert_eq!(BCryptDestroyHash(h_cmac), STATUS_SUCCESS);
            }
            assert_eq!(BCryptCloseAlgorithmProvider(h_cmac_alg, 0), STATUS_SUCCESS);

            // AES-GMAC matches GCM tag with empty plaintext
            let mut h_gmac_alg = std::ptr::null_mut();
            assert_eq!(BCryptOpenAlgorithmProvider(&mut h_gmac_alg, w("AES-GMAC").as_ptr(), std::ptr::null(), 0), STATUS_SUCCESS);
            let mut h_gmac = std::ptr::null_mut();
            let gmac_key = hex("feffe9928665731c6d6a8f9467308308");
            let gmac_iv = hex("cafebabefacedbaddecaf888");
            let gmac_aad = hex("feedfacedeadbeeffeedfacedeadbeefabaddad2");
            assert_eq!(BCryptCreateHash(h_gmac_alg, &mut h_gmac, std::ptr::null_mut(), 0, gmac_key.as_ptr() as *mut u8, 16, 0), STATUS_SUCCESS);
            assert_eq!(BCryptSetProperty(h_gmac, w("IV").as_ptr(), gmac_iv.as_ptr(), gmac_iv.len() as u32, 0), STATUS_SUCCESS);
            assert_eq!(BCryptHashData(h_gmac, gmac_aad.as_ptr() as *mut u8, gmac_aad.len() as u32, 0), STATUS_SUCCESS);
            let mut gmac_tag = [0u8; 16];
            assert_eq!(BCryptFinishHash(h_gmac, gmac_tag.as_mut_ptr(), 16, 0), STATUS_SUCCESS);
            let (_, expected_gmac) = aes_gcm_crypt(true, &gmac_key, &gmac_iv, &gmac_aad, &[], 16, None).unwrap();
            assert_eq!(gmac_tag.as_slice(), expected_gmac.as_slice());
            assert_eq!(BCryptDestroyHash(h_gmac), STATUS_SUCCESS);
            assert_eq!(BCryptCloseAlgorithmProvider(h_gmac_alg, 0), STATUS_SUCCESS);
        }
    }

    #[test]
    fn aes_ccm_rfc3610_and_aes_key_wrap_rfc3394_vectors() {
        unsafe {
            let w = |s: &str| s.encode_utf16().chain([0]).collect::<Vec<u16>>();
            let wbytes = |s: &str| -> Vec<u8> { w(s).into_iter().flat_map(u16::to_le_bytes).collect() };

            // --- RFC 3610 Packet Vector #1 (AES-128-CCM) ---
            let mut h_aes = std::ptr::null_mut();
            assert_eq!(BCryptOpenAlgorithmProvider(&mut h_aes, w("AES").as_ptr(), std::ptr::null(), 0), STATUS_SUCCESS);
            let mode_ccm = wbytes("ChainingModeCCM");
            assert_eq!(BCryptSetProperty(h_aes, w("ChainingMode").as_ptr(), mode_ccm.as_ptr(), mode_ccm.len() as u32, 0), STATUS_SUCCESS);

            // CCM AuthTagLength must report [4, 16, 2]
            let mut tag_lens = [0u32; 3];
            let mut got = 0u32;
            assert_eq!(BCryptGetProperty(h_aes, w("AuthTagLength").as_ptr(), tag_lens.as_mut_ptr() as *mut u8, 12, &mut got, 0), STATUS_SUCCESS);
            assert_eq!(tag_lens, [4, 16, 2]);

            let key = hex("c0c1c2c3c4c5c6c7c8c9cacbcccdcecf");
            let mut nonce = hex("00000003020100a0a1a2a3a4a5");
            let aad = hex("0001020304050607");
            let pt = hex("08090a0b0c0d0e0f101112131415161718191a1b1c1d1e");
            let expect_ct = hex("588c979a61c663d2f066d0c2c0f989806d5f6b61dac384");
            let expect_tag = hex("17e8d12cfdf926e0");

            let mut h_ccm = std::ptr::null_mut();
            assert_eq!(BCryptGenerateSymmetricKey(h_aes, &mut h_ccm, std::ptr::null_mut(), 0, key.as_ptr(), 16, 0), STATUS_SUCCESS);
            let mut tag = [0u8; 8];
            let mut auth = BcryptAuthCipherModeInfo {
                cb_size: std::mem::size_of::<BcryptAuthCipherModeInfo>() as u32,
                dw_info_version: 1,
                pb_nonce: nonce.as_mut_ptr(),
                cb_nonce: nonce.len() as u32,
                pb_auth_data: aad.as_ptr() as *mut u8,
                cb_auth_data: aad.len() as u32,
                pb_tag: tag.as_mut_ptr(),
                cb_tag: 8,
                pb_mac_context: std::ptr::null_mut(),
                cb_mac_context: 0,
                cb_aad: 0,
                cb_data: 0,
                dw_flags: 0,
            };
            let mut ct = vec![0u8; pt.len()];
            let mut out_len = 0u32;
            assert_eq!(
                BCryptEncrypt(
                    h_ccm,
                    pt.as_ptr(),
                    pt.len() as u32,
                    &mut auth as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                    0,
                    ct.as_mut_ptr(),
                    ct.len() as u32,
                    &mut out_len,
                    0,
                ),
                STATUS_SUCCESS
            );
            assert_eq!(ct, expect_ct);
            assert_eq!(tag.as_slice(), expect_tag.as_slice());

            let mut dec = vec![0u8; ct.len()];
            assert_eq!(
                BCryptDecrypt(
                    h_ccm,
                    ct.as_ptr(),
                    ct.len() as u32,
                    &mut auth as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                    0,
                    dec.as_mut_ptr(),
                    dec.len() as u32,
                    &mut out_len,
                    0,
                ),
                STATUS_SUCCESS
            );
            assert_eq!(dec, pt);

            // Tampered tag -> STATUS_AUTH_TAG_MISMATCH
            let mut bad_tag = tag;
            bad_tag[7] ^= 0x01;
            auth.pb_tag = bad_tag.as_mut_ptr();
            assert_eq!(
                BCryptDecrypt(
                    h_ccm,
                    ct.as_ptr(),
                    ct.len() as u32,
                    &mut auth as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                    0,
                    dec.as_mut_ptr(),
                    dec.len() as u32,
                    &mut out_len,
                    0,
                ),
                STATUS_AUTH_TAG_MISMATCH
            );
            assert_eq!(BCryptDestroyKey(h_ccm), STATUS_SUCCESS);

            // --- RFC 3394 AES Key Wrap (128-bit and 256-bit) via BCryptExportKey/BCryptImportKey ---
            let mode_ecb = wbytes("ChainingModeECB");
            assert_eq!(BCryptSetProperty(h_aes, w("ChainingMode").as_ptr(), mode_ecb.as_ptr(), mode_ecb.len() as u32, 0), STATUS_SUCCESS);
            for (kek_hex, data_hex, expect_wrap_hex) in [
                (
                    "000102030405060708090a0b0c0d0e0f",
                    "00112233445566778899aabbccddeeff",
                    "1fa68b0a8112b447aef34bd8fb5a7b829d3e862371d2cfe5",
                ),
                (
                    "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
                    "00112233445566778899aabbccddeeff000102030405060708090a0b0c0d0e0f",
                    "28c9f404c4b810f4cbccb35cfb87f8263f5786e2d80ed326cbc7f0e71a99f43bfb988b9b7a02dd21",
                ),
            ] {
                let kek = hex(kek_hex);
                let target = hex(data_hex);
                let mut h_kek = std::ptr::null_mut();
                let mut h_target = std::ptr::null_mut();
                assert_eq!(BCryptGenerateSymmetricKey(h_aes, &mut h_kek, std::ptr::null_mut(), 0, kek.as_ptr(), kek.len() as u32, 0), STATUS_SUCCESS);
                assert_eq!(BCryptGenerateSymmetricKey(h_aes, &mut h_target, std::ptr::null_mut(), 0, target.as_ptr(), target.len() as u32, 0), STATUS_SUCCESS);

                let mut wlen = 0u32;
                assert_eq!(
                    BCryptExportKey(h_target, h_kek, w("Rfc3565KeyWrapBlob").as_ptr(), std::ptr::null_mut(), 0, &mut wlen, 0),
                    STATUS_SUCCESS
                );
                let mut wrapped = vec![0u8; wlen as usize];
                assert_eq!(
                    BCryptExportKey(h_target, h_kek, w("Rfc3565KeyWrapBlob").as_ptr(), wrapped.as_mut_ptr(), wlen, &mut wlen, 0),
                    STATUS_SUCCESS
                );
                assert_eq!(wrapped, hex(expect_wrap_hex));

                let mut h_unwrapped = std::ptr::null_mut();
                assert_eq!(
                    BCryptImportKey(h_aes, h_kek, w("Rfc3565KeyWrapBlob").as_ptr(), &mut h_unwrapped, std::ptr::null_mut(), 0, wrapped.as_ptr(), wlen, 0),
                    STATUS_SUCCESS
                );
                // Re-export unwrapped key as KeyDataBlob and verify exact key material matches.
                let mut klen = 0u32;
                assert_eq!(BCryptExportKey(h_unwrapped, std::ptr::null_mut(), w("KeyDataBlob").as_ptr(), std::ptr::null_mut(), 0, &mut klen, 0), STATUS_SUCCESS);
                let mut kblob = vec![0u8; klen as usize];
                assert_eq!(BCryptExportKey(h_unwrapped, std::ptr::null_mut(), w("KeyDataBlob").as_ptr(), kblob.as_mut_ptr(), klen, &mut klen, 0), STATUS_SUCCESS);
                assert_eq!(&kblob[12..], target.as_slice());

                // Tampered wrap blob fails integrity check with STATUS_DATA_ERROR
                let mut bad_wrap = wrapped.clone();
                bad_wrap[0] ^= 0x01;
                let mut h_bad = std::ptr::null_mut();
                assert_eq!(
                    BCryptImportKey(h_aes, h_kek, w("Rfc3565KeyWrapBlob").as_ptr(), &mut h_bad, std::ptr::null_mut(), 0, bad_wrap.as_ptr(), wlen, 0),
                    STATUS_DATA_ERROR
                );

                assert_eq!(BCryptDestroyKey(h_unwrapped), STATUS_SUCCESS);
                assert_eq!(BCryptDestroyKey(h_target), STATUS_SUCCESS);
                assert_eq!(BCryptDestroyKey(h_kek), STATUS_SUCCESS);
            }
            assert_eq!(BCryptCloseAlgorithmProvider(h_aes, 0), STATUS_SUCCESS);
        }
    }

    #[test]
    fn des_3des_rc2_rc4_known_answer_vectors() {
        unsafe {
            let w = |s: &str| s.encode_utf16().chain([0]).collect::<Vec<u16>>();
            let wbytes = |s: &str| -> Vec<u8> { w(s).into_iter().flat_map(u16::to_le_bytes).collect() };
            let mode_ecb = wbytes("ChainingModeECB");

            // --- DES ECB known-answer vector ---
            let mut h_des_alg = std::ptr::null_mut();
            assert_eq!(BCryptOpenAlgorithmProvider(&mut h_des_alg, w("DES").as_ptr(), std::ptr::null(), 0), STATUS_SUCCESS);
            assert_eq!(BCryptSetProperty(h_des_alg, w("ChainingMode").as_ptr(), mode_ecb.as_ptr(), mode_ecb.len() as u32, 0), STATUS_SUCCESS);
            let des_key = hex("133457799bbcdff1");
            let des_pt = hex("0123456789abcdef");
            let mut h_des = std::ptr::null_mut();
            assert_eq!(BCryptGenerateSymmetricKey(h_des_alg, &mut h_des, std::ptr::null_mut(), 0, des_key.as_ptr(), 8, 0), STATUS_SUCCESS);
            let mut ct = [0u8; 8];
            let mut out_len = 0u32;
            assert_eq!(BCryptEncrypt(h_des, des_pt.as_ptr(), 8, std::ptr::null_mut(), std::ptr::null_mut(), 0, ct.as_mut_ptr(), 8, &mut out_len, 0), STATUS_SUCCESS);
            assert_eq!(ct.as_slice(), hex("85e813540f0ab405").as_slice());
            let mut dec = [0u8; 8];
            assert_eq!(BCryptDecrypt(h_des, ct.as_ptr(), 8, std::ptr::null_mut(), std::ptr::null_mut(), 0, dec.as_mut_ptr(), 8, &mut out_len, 0), STATUS_SUCCESS);
            assert_eq!(dec.as_slice(), des_pt.as_slice());
            assert_eq!(BCryptDestroyKey(h_des), STATUS_SUCCESS);
            assert_eq!(BCryptCloseAlgorithmProvider(h_des_alg, 0), STATUS_SUCCESS);

            // --- 3DES & 3DES_112 CBC + PKCS7 + OpaqueKeyBlob v2 roundtrip ---
            let mut h_3des_alg = std::ptr::null_mut();
            assert_eq!(BCryptOpenAlgorithmProvider(&mut h_3des_alg, w("3DES").as_ptr(), std::ptr::null(), 0), STATUS_SUCCESS);
            let k3 = hex("0123456789abcdef23456789abcdef01456789abcdef0123");
            let mut h_3des = std::ptr::null_mut();
            assert_eq!(BCryptGenerateSymmetricKey(h_3des_alg, &mut h_3des, std::ptr::null_mut(), 0, k3.as_ptr(), 24, 0), STATUS_SUCCESS);
            let mut iv = hex("1122334455667788");
            let msg = b"Triple-DES CBC padding test!";
            let mut ct3 = vec![0u8; 40];
            assert_eq!(
                BCryptEncrypt(h_3des, msg.as_ptr(), msg.len() as u32, std::ptr::null_mut(), iv.as_mut_ptr(), 8, ct3.as_mut_ptr(), ct3.len() as u32, &mut out_len, BCRYPT_BLOCK_PADDING),
                STATUS_SUCCESS
            );
            assert_eq!(out_len, 32);
            // Export OpaqueKeyBlob and re-import to decrypt
            let mut olen = 0u32;
            assert_eq!(BCryptExportKey(h_3des, std::ptr::null_mut(), w("OpaqueKeyBlob").as_ptr(), std::ptr::null_mut(), 0, &mut olen, 0), STATUS_SUCCESS);
            let mut oblob = vec![0u8; olen as usize];
            assert_eq!(BCryptExportKey(h_3des, std::ptr::null_mut(), w("OpaqueKeyBlob").as_ptr(), oblob.as_mut_ptr(), olen, &mut olen, 0), STATUS_SUCCESS);
            let mut h_3des2 = std::ptr::null_mut();
            assert_eq!(BCryptImportKey(h_3des_alg, std::ptr::null_mut(), w("OpaqueKeyBlob").as_ptr(), &mut h_3des2, std::ptr::null_mut(), 0, oblob.as_ptr(), olen, 0), STATUS_SUCCESS);
            let mut iv2 = hex("1122334455667788");
            let mut pt3 = vec![0u8; 32];
            assert_eq!(
                BCryptDecrypt(h_3des2, ct3.as_ptr(), 32, std::ptr::null_mut(), iv2.as_mut_ptr(), 8, pt3.as_mut_ptr(), 32, &mut out_len, BCRYPT_BLOCK_PADDING),
                STATUS_SUCCESS
            );
            assert_eq!(&pt3[..out_len as usize], msg);
            assert_eq!(BCryptDestroyKey(h_3des2), STATUS_SUCCESS);
            assert_eq!(BCryptDestroyKey(h_3des), STATUS_SUCCESS);
            assert_eq!(BCryptCloseAlgorithmProvider(h_3des_alg, 0), STATUS_SUCCESS);

            // --- RFC 2268 Section 5 RC2 known-answer vectors ---
            let mut h_rc2_alg = std::ptr::null_mut();
            assert_eq!(BCryptOpenAlgorithmProvider(&mut h_rc2_alg, w("RC2").as_ptr(), std::ptr::null(), 0), STATUS_SUCCESS);
            assert_eq!(BCryptSetProperty(h_rc2_alg, w("ChainingMode").as_ptr(), mode_ecb.as_ptr(), mode_ecb.len() as u32, 0), STATUS_SUCCESS);
            for (eff_bits, key_hex, pt_hex, ct_hex) in [
                (63u32, "0000000000000000", "0000000000000000", "ebb773f993278eff"),
                (64u32, "ffffffffffffffff", "ffffffffffffffff", "278b27e42e2f0d49"),
                (128u32, "88bca90e90875a7f0f79c384627bafb2", "0000000000000000", "2269552ab0f85ca6"),
            ] {
                let k = hex(key_hex);
                let p = hex(pt_hex);
                let mut hk = std::ptr::null_mut();
                assert_eq!(BCryptGenerateSymmetricKey(h_rc2_alg, &mut hk, std::ptr::null_mut(), 0, k.as_ptr(), k.len() as u32, 0), STATUS_SUCCESS);
                assert_eq!(BCryptSetProperty(hk, w("EffectiveKeyLength").as_ptr(), eff_bits.to_le_bytes().as_ptr(), 4, 0), STATUS_SUCCESS);
                let mut c = [0u8; 8];
                assert_eq!(BCryptEncrypt(hk, p.as_ptr(), 8, std::ptr::null_mut(), std::ptr::null_mut(), 0, c.as_mut_ptr(), 8, &mut out_len, 0), STATUS_SUCCESS);
                assert_eq!(c.as_slice(), hex(ct_hex).as_slice());
                let mut d = [0u8; 8];
                assert_eq!(BCryptDecrypt(hk, c.as_ptr(), 8, std::ptr::null_mut(), std::ptr::null_mut(), 0, d.as_mut_ptr(), 8, &mut out_len, 0), STATUS_SUCCESS);
                assert_eq!(d.as_slice(), p.as_slice());
                assert_eq!(BCryptDestroyKey(hk), STATUS_SUCCESS);
            }
            assert_eq!(BCryptCloseAlgorithmProvider(h_rc2_alg, 0), STATUS_SUCCESS);

            // --- RC4 known-answer vector (Key="Key", Plaintext="Plaintext") + streaming ---
            let mut h_rc4_alg = std::ptr::null_mut();
            assert_eq!(BCryptOpenAlgorithmProvider(&mut h_rc4_alg, w("RC4").as_ptr(), std::ptr::null(), 0), STATUS_SUCCESS);
            let mut h_rc4 = std::ptr::null_mut();
            assert_eq!(BCryptGenerateSymmetricKey(h_rc4_alg, &mut h_rc4, std::ptr::null_mut(), 0, b"Key".as_ptr(), 3, 0), STATUS_SUCCESS);
            let mut ct_rc4 = [0u8; 9];
            assert_eq!(BCryptEncrypt(h_rc4, b"Plain".as_ptr(), 5, std::ptr::null_mut(), std::ptr::null_mut(), 0, ct_rc4.as_mut_ptr(), 5, &mut out_len, 0), STATUS_SUCCESS);
            assert_eq!(BCryptEncrypt(h_rc4, b"text".as_ptr(), 4, std::ptr::null_mut(), std::ptr::null_mut(), 0, ct_rc4[5..].as_mut_ptr(), 4, &mut out_len, 0), STATUS_SUCCESS);
            assert_eq!(ct_rc4.as_slice(), hex("bbf316e8d940af0ad3").as_slice());
            assert_eq!(BCryptDestroyKey(h_rc4), STATUS_SUCCESS);
            assert_eq!(BCryptCloseAlgorithmProvider(h_rc4_alg, 0), STATUS_SUCCESS);
        }
    }

    #[test]
    fn rsa_oaep_pkcs1_encrypt_decrypt_and_capi_blob_interop() {
        unsafe {
            let w = |s: &str| s.encode_utf16().chain([0]).collect::<Vec<u16>>();
            let mut h_rsa = std::ptr::null_mut();
            assert_eq!(BCryptOpenAlgorithmProvider(&mut h_rsa, w("RSA").as_ptr(), std::ptr::null(), 0), STATUS_SUCCESS);

            let mut h_key = std::ptr::null_mut();
            assert_eq!(BCryptGenerateKeyPair(h_rsa, &mut h_key, 2048, 0), STATUS_SUCCESS);
            assert_eq!(BCryptFinalizeKeyPair(h_key, 0), STATUS_SUCCESS);

            // Export as CAPIPUBLICBLOB & CAPIPRIVATEBLOB and re-import through BCryptImportKeyPair
            let mut pub_len = 0u32;
            assert_eq!(BCryptExportKey(h_key, std::ptr::null_mut(), w("CAPIPUBLICBLOB").as_ptr(), std::ptr::null_mut(), 0, &mut pub_len, 0), STATUS_SUCCESS);
            let mut pub_blob = vec![0u8; pub_len as usize];
            assert_eq!(BCryptExportKey(h_key, std::ptr::null_mut(), w("CAPIPUBLICBLOB").as_ptr(), pub_blob.as_mut_ptr(), pub_len, &mut pub_len, 0), STATUS_SUCCESS);
            assert_eq!(pub_blob[0], CAPI_PUBLICKEYBLOB);

            let mut priv_len = 0u32;
            assert_eq!(BCryptExportKey(h_key, std::ptr::null_mut(), w("CAPIPRIVATEBLOB").as_ptr(), std::ptr::null_mut(), 0, &mut priv_len, 0), STATUS_SUCCESS);
            let mut priv_blob = vec![0u8; priv_len as usize];
            assert_eq!(BCryptExportKey(h_key, std::ptr::null_mut(), w("CAPIPRIVATEBLOB").as_ptr(), priv_blob.as_mut_ptr(), priv_len, &mut priv_len, 0), STATUS_SUCCESS);
            assert_eq!(priv_blob[0], CAPI_PRIVATEKEYBLOB);

            let mut h_pub = std::ptr::null_mut();
            let mut h_priv = std::ptr::null_mut();
            assert_eq!(BCryptImportKeyPair(h_rsa, std::ptr::null_mut(), w("CAPIPUBLICBLOB").as_ptr(), &mut h_pub, pub_blob.as_mut_ptr(), pub_len, 0), STATUS_SUCCESS);
            assert_eq!(BCryptImportKeyPair(h_rsa, std::ptr::null_mut(), w("CAPIPRIVATEBLOB").as_ptr(), &mut h_priv, priv_blob.as_mut_ptr(), priv_len, 0), STATUS_SUCCESS);

            // Windows contract: non-null hImportKey to BCryptImportKeyPair -> STATUS_NOT_SUPPORTED
            let mut h_dummy = std::ptr::null_mut();
            assert_eq!(
                BCryptImportKeyPair(h_rsa, h_key, w("CAPIPUBLICBLOB").as_ptr(), &mut h_dummy, pub_blob.as_mut_ptr(), pub_len, 0),
                STATUS_NOT_SUPPORTED
            );

            // RSAES-OAEP with SHA-256 + custom label
            let alg_w = w("SHA256");
            let label = b"oaep-custom-label";
            let mut oaep = BcryptOaepPaddingInfo {
                psz_alg_id: alg_w.as_ptr(),
                pb_label: label.as_ptr(),
                cb_label: label.len() as u32,
            };
            let msg = b"hello rsa oaep & pkcs1";
            let mut ct_len = 0u32;
            assert_eq!(
                BCryptEncrypt(h_pub, msg.as_ptr(), msg.len() as u32, &mut oaep as *mut _ as *mut c_void, std::ptr::null_mut(), 0, std::ptr::null_mut(), 0, &mut ct_len, BCRYPT_PAD_OAEP),
                STATUS_SUCCESS
            );
            assert_eq!(ct_len, 256);
            let mut ct = vec![0u8; 256];
            assert_eq!(
                BCryptEncrypt(h_pub, msg.as_ptr(), msg.len() as u32, &mut oaep as *mut _ as *mut c_void, std::ptr::null_mut(), 0, ct.as_mut_ptr(), 256, &mut ct_len, BCRYPT_PAD_OAEP),
                STATUS_SUCCESS
            );
            let mut dec = vec![0u8; 256];
            let mut dec_len = 0u32;
            assert_eq!(
                BCryptDecrypt(h_priv, ct.as_ptr(), 256, &mut oaep as *mut _ as *mut c_void, std::ptr::null_mut(), 0, dec.as_mut_ptr(), 256, &mut dec_len, BCRYPT_PAD_OAEP),
                STATUS_SUCCESS
            );
            assert_eq!(&dec[..dec_len as usize], msg);

            // Wrong OAEP label must fail decryption
            let wrong_label = b"wrong-label";
            let mut bad_oaep = BcryptOaepPaddingInfo {
                psz_alg_id: alg_w.as_ptr(),
                pb_label: wrong_label.as_ptr(),
                cb_label: wrong_label.len() as u32,
            };
            assert_eq!(
                BCryptDecrypt(h_priv, ct.as_ptr(), 256, &mut bad_oaep as *mut _ as *mut c_void, std::ptr::null_mut(), 0, dec.as_mut_ptr(), 256, &mut dec_len, BCRYPT_PAD_OAEP),
                STATUS_DATA_ERROR
            );

            // RSAES-PKCS1-v1_5 encrypt & decrypt
            let mut ct2 = vec![0u8; 256];
            assert_eq!(
                BCryptEncrypt(h_pub, msg.as_ptr(), msg.len() as u32, std::ptr::null_mut(), std::ptr::null_mut(), 0, ct2.as_mut_ptr(), 256, &mut ct_len, BCRYPT_PAD_PKCS1),
                STATUS_SUCCESS
            );
            assert_eq!(
                BCryptDecrypt(h_priv, ct2.as_ptr(), 256, std::ptr::null_mut(), std::ptr::null_mut(), 0, dec.as_mut_ptr(), 256, &mut dec_len, BCRYPT_PAD_PKCS1),
                STATUS_SUCCESS
            );
            assert_eq!(&dec[..dec_len as usize], msg);

            // BCRYPT_PAD_PKCS1_OPTIONAL_HASH_OID verifies raw (OID-less) PKCS#1 signature even when SHA256 is specified
            let mut h = Digest::Sha256.hash(b"optional oid test");
            let mut raw_pad: *const u16 = std::ptr::null();
            let mut sig = vec![0u8; 256];
            let mut slen = 0u32;
            assert_eq!(
                BCryptSignHash(h_priv, &mut raw_pad as *mut _ as *mut c_void, h.as_ptr(), 32, sig.as_mut_ptr(), 256, &mut slen, BCRYPT_PAD_PKCS1),
                STATUS_SUCCESS
            );
            let mut sha256_pad: *const u16 = alg_w.as_ptr();
            assert_eq!(
                BCryptVerifySignature(h_pub, &mut sha256_pad as *mut _ as *mut c_void, h.as_mut_ptr(), 32, sig.as_mut_ptr(), 256, BCRYPT_PAD_PKCS1),
                STATUS_INVALID_SIGNATURE
            );
            assert_eq!(
                BCryptVerifySignature(
                    h_pub,
                    &mut sha256_pad as *mut _ as *mut c_void,
                    h.as_mut_ptr(),
                    32,
                    sig.as_mut_ptr(),
                    256,
                    BCRYPT_PAD_PKCS1 | BCRYPT_PAD_PKCS1_OPTIONAL_HASH_OID
                ),
                STATUS_SUCCESS
            );

            assert_eq!(BCryptDestroyKey(h_priv), STATUS_SUCCESS);
            assert_eq!(BCryptDestroyKey(h_pub), STATUS_SUCCESS);
            assert_eq!(BCryptDestroyKey(h_key), STATUS_SUCCESS);
            assert_eq!(BCryptCloseAlgorithmProvider(h_rsa, 0), STATUS_SUCCESS);
        }
    }

    #[test]
    fn ecdh_p256_rfc5903_vector_and_kdf_derivations() {
        unsafe {
            let w = |s: &str| s.encode_utf16().chain([0]).collect::<Vec<u16>>();
            let wbytes = |s: &str| -> Vec<u8> { w(s).into_iter().flat_map(u16::to_le_bytes).collect() };

            // RFC 5903 Section 8.1 (256-bit Random ECP Group / NIST P-256) known-answer vector
            let mut h_ecdh = std::ptr::null_mut();
            assert_eq!(BCryptOpenAlgorithmProvider(&mut h_ecdh, w("ECDH_P256").as_ptr(), std::ptr::null(), 0), STATUS_SUCCESS);

            // Check ECCCurveNameList property
            let mut list_len = 0u32;
            assert_eq!(BCryptGetProperty(h_ecdh, w("ECCCurveNameList").as_ptr(), std::ptr::null_mut(), 0, &mut list_len, 0), STATUS_SUCCESS);
            let mut list_buf = vec![0u8; list_len as usize];
            assert_eq!(BCryptGetProperty(h_ecdh, w("ECCCurveNameList").as_ptr(), list_buf.as_mut_ptr(), list_len, &mut list_len, 0), STATUS_SUCCESS);
            let count = u32::from_le_bytes(list_buf[0..4].try_into().unwrap());
            assert_eq!(count, 3);

            // Alice private key (dA, xA, yA) and Bob public key (xB, yB) from RFC 5903 §8.1
            let mut alice_priv_blob = Vec::new();
            alice_priv_blob.extend_from_slice(&BCRYPT_ECDH_PRIVATE_P256_MAGIC.to_le_bytes());
            alice_priv_blob.extend_from_slice(&32u32.to_le_bytes());
            alice_priv_blob.extend_from_slice(&hex("62d5bd3372af75fe85a040715d0f502428e07046868b0bfdfa61d731afe44f26"));
            alice_priv_blob.extend_from_slice(&hex("ac333a93a9e70a81cd5a95b5bf8d13990eb741c8c38872b4a07d275a014e30cf"));
            alice_priv_blob.extend_from_slice(&hex("c88f01f510d9ac3f70a292daa2316de544e9aab8afe84049c62a9c57862d1433"));

            let mut bob_pub_blob = Vec::new();
            bob_pub_blob.extend_from_slice(&BCRYPT_ECDH_PUBLIC_P256_MAGIC.to_le_bytes());
            bob_pub_blob.extend_from_slice(&32u32.to_le_bytes());
            bob_pub_blob.extend_from_slice(&hex("d12dfb5289c8d4f81208b70270398c342296970a0bccb74c736fc7554494bf63"));
            bob_pub_blob.extend_from_slice(&hex("56fbf3ca366cc23e8157854c13c58d6aac23f046ada30f8353e74f33039872ab"));

            let expected_z_be = hex("d6840f6b42f6edafd13116e0e12565202fef8e9ece7dce03812464d04b9442de");
            let mut expected_z_le = expected_z_be.clone();
            expected_z_le.reverse();

            let mut h_alice = std::ptr::null_mut();
            let mut h_bob = std::ptr::null_mut();
            assert_eq!(
                BCryptImportKeyPair(h_ecdh, std::ptr::null_mut(), w("ECCPRIVATEBLOB").as_ptr(), &mut h_alice, alice_priv_blob.as_mut_ptr(), alice_priv_blob.len() as u32, 0),
                STATUS_SUCCESS
            );
            assert_eq!(
                BCryptImportKeyPair(h_ecdh, std::ptr::null_mut(), w("ECCPUBLICBLOB").as_ptr(), &mut h_bob, bob_pub_blob.as_mut_ptr(), bob_pub_blob.len() as u32, 0),
                STATUS_SUCCESS
            );

            let mut h_sec = std::ptr::null_mut();
            assert_eq!(BCryptSecretAgreement(h_alice, h_bob, &mut h_sec, 0), STATUS_SUCCESS);

            // 1. BCRYPT_KDF_RAW_SECRET ("TRUNCATE") -> little-endian shared secret
            let mut raw = [0u8; 32];
            let mut got = 0u32;
            assert_eq!(
                BCryptDeriveKey(h_sec, w("TRUNCATE").as_ptr(), std::ptr::null(), raw.as_mut_ptr(), 32, &mut got, 0),
                STATUS_SUCCESS
            );
            assert_eq!(got, 32);
            assert_eq!(raw.as_slice(), expected_z_le.as_slice());

            // 2. BCRYPT_KDF_HASH ("HASH") with SHA-256 + prepend/append
            let mut sha256_w = wbytes("SHA256");
            let mut pre = b"PRE".to_vec();
            let mut app = b"APP".to_vec();
            let mut bufs = [
                BcryptBuffer { cb_buffer: sha256_w.len() as u32, buffer_type: KDF_HASH_ALGORITHM, pv_buffer: sha256_w.as_mut_ptr() as *mut c_void },
                BcryptBuffer { cb_buffer: pre.len() as u32, buffer_type: KDF_SECRET_PREPEND, pv_buffer: pre.as_mut_ptr() as *mut c_void },
                BcryptBuffer { cb_buffer: app.len() as u32, buffer_type: KDF_SECRET_APPEND, pv_buffer: app.as_mut_ptr() as *mut c_void },
            ];
            let desc = BcryptBufferDesc { ul_version: 0, c_buffers: bufs.len() as u32, p_buffers: bufs.as_mut_ptr() };
            let mut dk = [0u8; 32];
            assert_eq!(
                BCryptDeriveKey(h_sec, w("HASH").as_ptr(), &desc as *const _ as *const c_void, dk.as_mut_ptr(), 32, &mut got, 0),
                STATUS_SUCCESS
            );
            assert_eq!(dk.as_slice(), Digest::Sha256.hash_slices(&[b"PRE", &expected_z_be, b"APP"]).as_slice());

            // 3. BCRYPT_KDF_SP80056A_CONCAT
            let mut party_u = b"Alice".to_vec();
            let mut party_v = b"Bob".to_vec();
            let mut concat_bufs = [
                BcryptBuffer { cb_buffer: sha256_w.len() as u32, buffer_type: KDF_HASH_ALGORITHM, pv_buffer: sha256_w.as_mut_ptr() as *mut c_void },
                BcryptBuffer { cb_buffer: party_u.len() as u32, buffer_type: KDF_PARTYUINFO, pv_buffer: party_u.as_mut_ptr() as *mut c_void },
                BcryptBuffer { cb_buffer: party_v.len() as u32, buffer_type: KDF_PARTYVINFO, pv_buffer: party_v.as_mut_ptr() as *mut c_void },
            ];
            let concat_desc = BcryptBufferDesc { ul_version: 0, c_buffers: concat_bufs.len() as u32, p_buffers: concat_bufs.as_mut_ptr() };
            let mut ck = [0u8; 32];
            assert_eq!(
                BCryptDeriveKey(h_sec, w("SP800_56A_CONCAT").as_ptr(), &concat_desc as *const _ as *const c_void, ck.as_mut_ptr(), 32, &mut got, 0),
                STATUS_SUCCESS
            );
            assert_eq!(ck.as_slice(), Digest::Sha256.hash_slices(&[&1u32.to_be_bytes(), &expected_z_be, b"Alice", b"Bob"]).as_slice());

            assert_eq!(BCryptDestroySecret(h_sec), STATUS_SUCCESS);
            assert_eq!(BCryptDestroyKey(h_bob), STATUS_SUCCESS);
            assert_eq!(BCryptDestroyKey(h_alice), STATUS_SUCCESS);
            assert_eq!(BCryptCloseAlgorithmProvider(h_ecdh, 0), STATUS_SUCCESS);

            // End-to-end ECDH_P384 keygen agreement (both sides derive identical secret)
            let mut h_p384 = std::ptr::null_mut();
            assert_eq!(BCryptOpenAlgorithmProvider(&mut h_p384, w("ECDH_P384").as_ptr(), std::ptr::null(), 0), STATUS_SUCCESS);
            let mut ka = std::ptr::null_mut();
            let mut kb = std::ptr::null_mut();
            assert_eq!(BCryptGenerateKeyPair(h_p384, &mut ka, 384, 0), STATUS_SUCCESS);
            assert_eq!(BCryptFinalizeKeyPair(ka, 0), STATUS_SUCCESS);
            assert_eq!(BCryptGenerateKeyPair(h_p384, &mut kb, 384, 0), STATUS_SUCCESS);
            assert_eq!(BCryptFinalizeKeyPair(kb, 0), STATUS_SUCCESS);

            let mut sa = std::ptr::null_mut();
            let mut sb = std::ptr::null_mut();
            assert_eq!(BCryptSecretAgreement(ka, kb, &mut sa, 0), STATUS_SUCCESS);
            assert_eq!(BCryptSecretAgreement(kb, ka, &mut sb, 0), STATUS_SUCCESS);
            let mut da = [0u8; 48];
            let mut db = [0u8; 48];
            assert_eq!(BCryptDeriveKey(sa, w("TRUNCATE").as_ptr(), std::ptr::null(), da.as_mut_ptr(), 48, &mut got, 0), STATUS_SUCCESS);
            assert_eq!(BCryptDeriveKey(sb, w("TRUNCATE").as_ptr(), std::ptr::null(), db.as_mut_ptr(), 48, &mut got, 0), STATUS_SUCCESS);
            assert_eq!(da, db);

            assert_eq!(BCryptDestroySecret(sa), STATUS_SUCCESS);
            assert_eq!(BCryptDestroySecret(sb), STATUS_SUCCESS);
            assert_eq!(BCryptDestroyKey(ka), STATUS_SUCCESS);
            assert_eq!(BCryptDestroyKey(kb), STATUS_SUCCESS);
            assert_eq!(BCryptCloseAlgorithmProvider(h_p384, 0), STATUS_SUCCESS);
        }
    }
}
