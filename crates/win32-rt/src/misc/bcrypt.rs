//! bcrypt.dll (CNG): random numbers, hashing (MD5, SHA-1/256/384/512, with
//! optional HMAC), PBKDF2, AES-128/192/256 (ECB, CBC, CFB, CTR, GCM),
//! RSA (PKCS#1 v1.5 and PSS keygen/import/export/sign/verify), and
//! ECDSA P-256 / P-384 / P-521 (keygen/import/export/sign/verify).
//!
//! Digests and AES block primitives use CommonCrypto (libSystem); asymmetric
//! keygen, ECDSA sign/verify, and RSA private-key operations use Apple's
//! `Security.framework` (`SecKey`), which is available identically on macOS and
//! iOS. RSA public-key verification also retains a pure-Rust big-integer
//! `modexp` path supporting arbitrary PSS salt lengths and 512/1024-bit keys.

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
const BCRYPT_BLOCK_PADDING: u32 = 0x1;
const BCRYPT_PAD_PKCS1: u32 = 0x2;
const BCRYPT_PAD_PSS: u32 = 0x8;
const BCRYPT_SUPPORTED_PAD_PKCS1_SIG: u32 = 0x0000_0004;
const BCRYPT_SUPPORTED_PAD_PSS: u32 = 0x0000_0010;

const BCRYPT_RSAPUBLIC_MAGIC: u32 = 0x3141_5352; // "RSA1"
const BCRYPT_RSAPRIVATE_MAGIC: u32 = 0x3241_5352; // "RSA2"
const BCRYPT_RSAFULLPRIVATE_MAGIC: u32 = 0x3341_5352; // "RSA3"
const BCRYPT_ECDSA_PUBLIC_P256_MAGIC: u32 = 0x3153_4345; // "ECS1"
const BCRYPT_ECDSA_PRIVATE_P256_MAGIC: u32 = 0x3253_4345; // "ECS2"
const BCRYPT_ECDSA_PUBLIC_P384_MAGIC: u32 = 0x3353_4345; // "ECS3"
const BCRYPT_ECDSA_PRIVATE_P384_MAGIC: u32 = 0x3453_4345; // "ECS4"
const BCRYPT_ECDSA_PUBLIC_P521_MAGIC: u32 = 0x3553_4345; // "ECS5"
const BCRYPT_ECDSA_PRIVATE_P521_MAGIC: u32 = 0x3653_4345; // "ECS6"
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
    Md5,
    Sha1,
    Sha256,
    Sha384,
    Sha512,
}

extern "C" {
    fn CC_MD5(data: *const c_void, len: u32, md: *mut u8) -> *mut u8;
    fn CC_SHA1(data: *const c_void, len: u32, md: *mut u8) -> *mut u8;
    fn CC_SHA256(data: *const c_void, len: u32, md: *mut u8) -> *mut u8;
    fn CC_SHA384(data: *const c_void, len: u32, md: *mut u8) -> *mut u8;
    fn CC_SHA512(data: *const c_void, len: u32, md: *mut u8) -> *mut u8;
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
    fn from_name(n: &str) -> Option<Digest> {
        Some(match n.to_ascii_uppercase().as_str() {
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
            Digest::Md5 => "MD5",
            Digest::Sha1 => "SHA1",
            Digest::Sha256 => "SHA256",
            Digest::Sha384 => "SHA384",
            Digest::Sha512 => "SHA512",
        }
    }
    pub fn len(self) -> usize {
        match self {
            Digest::Md5 => 16,
            Digest::Sha1 => 20,
            Digest::Sha256 => 32,
            Digest::Sha384 => 48,
            Digest::Sha512 => 64,
        }
    }
    fn block_len(self) -> u32 {
        match self {
            Digest::Sha384 | Digest::Sha512 => 128,
            _ => 64,
        }
    }
    pub fn hash(self, data: &[u8]) -> Vec<u8> {
        let mut out = vec![0u8; self.len()];
        let f = match self {
            Digest::Md5 => CC_MD5,
            Digest::Sha1 => CC_SHA1,
            Digest::Sha256 => CC_SHA256,
            Digest::Sha384 => CC_SHA384,
            Digest::Sha512 => CC_SHA512,
        };
        // CC_* take a u32 length; hash in one call (inputs here are small).
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
        let alg = match self {
            Digest::Sha1 => 0,
            Digest::Md5 => 1,
            Digest::Sha256 => 2,
            Digest::Sha384 => 3,
            Digest::Sha512 => 4,
        };
        let mut out = vec![0u8; self.len()];
        unsafe {
            CCHmac(alg, key.as_ptr() as *const c_void, key.len(), data.as_ptr() as *const c_void, data.len(), out.as_mut_ptr())
        };
        out
    }
    /// DER DigestInfo prefix for EMSA-PKCS1-v1_5.
    fn digest_info_prefix(self) -> &'static [u8] {
        match self {
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
struct RsaKey {
    pub_key: RsaPublicKey,
    priv_params: Option<RsaPrivateParams>,
}

impl RsaPublicKey {
    /// Upper bound on RSA public modulus length in bytes (8192 bits) for in-tree verification.
    pub const MAX_MODULUS_BYTES: usize = 1024;
    /// Minimum RSA modulus length in bytes (512 bits) accepted for X.509/CMS certificates.
    pub const MIN_ASN1_MODULUS_BYTES: usize = 64;
    /// Upper bound on RSA public exponent length in bytes (64 bits, matching Windows CNG / OpenSSL).
    pub const MAX_PUBLIC_EXPONENT_BYTES: usize = 8;

    fn strip_leading_zeros(mut bytes: &[u8]) -> &[u8] {
        while !bytes.is_empty() && bytes[0] == 0 {
            bytes = &bytes[1..];
        }
        bytes
    }

    /// Validate RSA public `(n, e)` big-endian byte slices before allocation or modular
    /// exponentiation: odd modulus `n >= 3` within `MAX_MODULUS_BYTES` (8192 bits) and odd
    /// exponent `e >= 3` within `MAX_PUBLIC_EXPONENT_BYTES` (64 bits).
    pub fn validate_public_params<'a>(n_be: &'a [u8], e_be: &'a [u8]) -> Option<(&'a [u8], &'a [u8])> {
        let n = Self::strip_leading_zeros(n_be);
        let e = Self::strip_leading_zeros(e_be);
        if n.is_empty()
            || n.len() > Self::MAX_MODULUS_BYTES
            || (n[n.len() - 1] & 1) == 0
            || (n.len() == 1 && n[0] < 3)
        {
            return None;
        }
        if e.is_empty()
            || e.len() > Self::MAX_PUBLIC_EXPONENT_BYTES
            || (e[e.len() - 1] & 1) == 0
            || (e.len() == 1 && e[0] < 3)
        {
            return None;
        }
        Some((n, e))
    }

    /// Construct a bounded `RsaPublicKey` from ASN.1 DER `RSAPublicKey` `INTEGER` content slices.
    /// Rejects negative integers, moduli outside 512..=8192 bits or even, and exponents outside
    /// odd `3..2^64` before copying any bytes.
    pub fn from_asn1_integers(n_der: &[u8], e_der: &[u8]) -> Option<RsaPublicKey> {
        if n_der.is_empty()
            || (n_der[0] & 0x80) != 0
            || e_der.is_empty()
            || (e_der[0] & 0x80) != 0
        {
            return None;
        }
        let (n, e) = Self::validate_public_params(n_der, e_der)?;
        if n.len() < Self::MIN_ASN1_MODULUS_BYTES {
            return None;
        }
        Some(RsaPublicKey {
            n: n.to_vec(),
            e: e.to_vec(),
        })
    }

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
        let e_raw = blob.get(24..24 + cb_exp)?;
        let n_raw = blob.get(24 + cb_exp..24 + cb_exp + cb_mod)?;
        let (n, e) = Self::validate_public_params(n_raw, e_raw)?;
        Some(RsaPublicKey {
            n: n.to_vec(),
            e: e.to_vec(),
        })
    }

    fn bits(&self) -> usize {
        bn_bits(&bn_from_be(&self.n))
    }

    fn k(&self) -> usize {
        self.bits().div_ceil(8)
    }

    /// RSAVP1: the encoded message `s^e mod n` (k bytes), or None if parameters are invalid or s >= n.
    fn open(&self, sig: &[u8]) -> Option<Vec<u8>> {
        let (n_be, e_be) = Self::validate_public_params(&self.n, &self.e)?;
        let k = n_be.len();
        if sig.len() > k {
            return None;
        }
        let n = bn_from_be(n_be);
        let s = bn_from_be(sig);
        if n.is_empty() || bn_cmp(&s, &n) != std::cmp::Ordering::Less {
            return None;
        }
        Some(bn_to_be(&bn_modexp(&s, &bn_from_be(e_be), &n), k))
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
    fn generate(bits: u32) -> Result<Self, i32> {
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

    fn from_blob(blob_type: &str, blob: &[u8]) -> Option<Self> {
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

    fn sign_raw_em(&self, em: &[u8]) -> Result<Vec<u8>, i32> {
        let der = self.to_pkcs1_private_der().ok_or(STATUS_INVALID_HANDLE)?;
        let sig = seckey::rsa_raw_sign(self.pub_key.bits() as u32, &der, em)?;
        pad_be(&sig, self.pub_key.k()).ok_or(STATUS_INVALID_PARAMETER)
    }

    fn sign_pkcs1(&self, digest: Option<Digest>, hash: &[u8]) -> Result<Vec<u8>, i32> {
        let em = self.pub_key.encode_pkcs1_em(digest, hash).ok_or(STATUS_INVALID_PARAMETER)?;
        self.sign_raw_em(&em)
    }

    fn sign_pss(&self, d: Digest, hash: &[u8], salt_len: usize) -> Result<Vec<u8>, i32> {
        let em = self.pub_key.encode_pss_em(d, hash, salt_len).ok_or(STATUS_INVALID_PARAMETER)?;
        self.sign_raw_em(&em)
    }
}

// ---------------------------------------------------------------------------
// ECDSA P-256 / P-384 / P-521 (Apple Security.framework SecKey backend)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EcdsaCurve {
    P256,
    P384,
    P521,
}

impl EcdsaCurve {
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

    fn pub_magic(self) -> u32 {
        match self {
            Self::P256 => BCRYPT_ECDSA_PUBLIC_P256_MAGIC,
            Self::P384 => BCRYPT_ECDSA_PUBLIC_P384_MAGIC,
            Self::P521 => BCRYPT_ECDSA_PUBLIC_P521_MAGIC,
        }
    }

    fn priv_magic(self) -> u32 {
        match self {
            Self::P256 => BCRYPT_ECDSA_PRIVATE_P256_MAGIC,
            Self::P384 => BCRYPT_ECDSA_PRIVATE_P384_MAGIC,
            Self::P521 => BCRYPT_ECDSA_PRIVATE_P521_MAGIC,
        }
    }

    fn win_curve_name(self) -> &'static str {
        match self {
            Self::P256 => "nistP256",
            Self::P384 => "nistP384",
            Self::P521 => "nistP521",
        }
    }

    fn alg_name(self) -> &'static str {
        match self {
            Self::P256 => "ECDSA_P256",
            Self::P384 => "ECDSA_P384",
            Self::P521 => "ECDSA_P521",
        }
    }
}

#[derive(Clone)]
struct EcdsaKey {
    curve: EcdsaCurve,
    x: Vec<u8>,
    y: Vec<u8>,
    d: Option<Vec<u8>>,
}

impl EcdsaKey {
    fn generate(curve: EcdsaCurve) -> Result<Self, i32> {
        let ext = seckey::generate_random_key_external(false, curve.bits())?;
        let cl = curve.coord_len();
        // Apple X9.63 private representation: 0x04 || X (cl) || Y (cl) || d (cl)
        if ext.len() != 1 + 3 * cl || ext[0] != 0x04 {
            return Err(STATUS_INVALID_PARAMETER);
        }
        Ok(EcdsaKey {
            curve,
            x: ext[1..1 + cl].to_vec(),
            y: ext[1 + cl..1 + 2 * cl].to_vec(),
            d: Some(ext[1 + 2 * cl..1 + 3 * cl].to_vec()),
        })
    }

    fn from_blob(blob_type: &str, expected_curve: Option<EcdsaCurve>, blob: &[u8]) -> Option<Self> {
        if blob.len() < 8 {
            return None;
        }
        let magic = u32::from_le_bytes(blob[0..4].try_into().unwrap());
        let cb_key = u32::from_le_bytes(blob[4..8].try_into().unwrap()) as usize;
        let (curve, is_priv) = match magic {
            BCRYPT_ECDSA_PUBLIC_P256_MAGIC => (EcdsaCurve::P256, false),
            BCRYPT_ECDSA_PRIVATE_P256_MAGIC => (EcdsaCurve::P256, true),
            BCRYPT_ECDSA_PUBLIC_P384_MAGIC => (EcdsaCurve::P384, false),
            BCRYPT_ECDSA_PRIVATE_P384_MAGIC => (EcdsaCurve::P384, true),
            BCRYPT_ECDSA_PUBLIC_P521_MAGIC => (EcdsaCurve::P521, false),
            BCRYPT_ECDSA_PRIVATE_P521_MAGIC => (EcdsaCurve::P521, true),
            _ => return None,
        };
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
                Some(EcdsaKey {
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
                Some(EcdsaKey {
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
        out.extend_from_slice(&self.curve.pub_magic().to_le_bytes());
        out.extend_from_slice(&(cl as u32).to_le_bytes());
        out.extend_from_slice(&self.x);
        out.extend_from_slice(&self.y);
        out
    }

    fn to_private_blob(&self) -> Option<Vec<u8>> {
        let d = self.d.as_ref()?;
        let cl = self.curve.coord_len();
        let mut out = Vec::with_capacity(8 + 3 * cl);
        out.extend_from_slice(&self.curve.priv_magic().to_le_bytes());
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
}

// ---------------------------------------------------------------------------
// AES-128/192/256 (CommonCrypto hardware AES + NIST SP 800-38D AES-GCM)
// ---------------------------------------------------------------------------

const KCC_ENCRYPT: u32 = 0;
const KCC_DECRYPT: u32 = 1;
const KCC_ALGORITHM_AES: u32 = 0;
const KCC_OPTION_PKCS7_PADDING: u32 = 1;
const KCC_OPTION_ECB_MODE: u32 = 2;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum AesMode {
    Ecb,
    Cbc,
    Cfb,
    Ctr,
    Gcm,
}

impl AesMode {
    fn from_name(s: &str) -> Option<Self> {
        let clean = s.trim_end_matches('\0');
        match clean {
            "ChainingModeECB" => Some(Self::Ecb),
            "ChainingModeCBC" => Some(Self::Cbc),
            "ChainingModeCFB" => Some(Self::Cfb),
            "ChainingModeCTR" => Some(Self::Ctr),
            "ChainingModeGCM" => Some(Self::Gcm),
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
        }
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

fn aes_cccrypt(encrypt: bool, ecb: bool, pkcs7: bool, key: &[u8], iv: Option<&[u8; 16]>, input: &[u8]) -> Result<Vec<u8>, i32> {
    if !pkcs7 && input.len() % 16 != 0 {
        return Err(STATUS_INVALID_BUFFER_SIZE);
    }
    if !encrypt && pkcs7 && (input.is_empty() || input.len() % 16 != 0) {
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
    let mut out = vec![0u8; input.len() + 16];
    let mut moved: usize = 0;
    let iv_ptr = iv.map_or(std::ptr::null(), |v| v.as_ptr() as *const c_void);
    let rc = unsafe {
        CCCrypt(
            op,
            KCC_ALGORITHM_AES,
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

fn aes_ctr_crypt(key: &[u8], iv: &mut [u8; 16], input: &[u8]) -> Result<Vec<u8>, i32> {
    let mut out = Vec::with_capacity(input.len());
    for chunk in input.chunks(16) {
        let ks = aes_encrypt_block(key, iv)?;
        for (b, k) in chunk.iter().zip(ks.iter()) {
            out.push(b ^ k);
        }
        // Increment 128-bit big-endian counter
        let c = u128::from_be_bytes(*iv).wrapping_add(1);
        *iv = c.to_be_bytes();
    }
    Ok(out)
}

fn aes_cfb_crypt(encrypt: bool, key: &[u8], iv: &mut [u8; 16], msg_block_len: usize, input: &[u8]) -> Result<Vec<u8>, i32> {
    let step = msg_block_len.clamp(1, 16);
    let mut out = Vec::with_capacity(input.len());
    for chunk in input.chunks(step) {
        let ks = aes_encrypt_block(key, iv)?;
        let n = chunk.len();
        let mut ct = [0u8; 16];
        for i in 0..n {
            let y = chunk[i] ^ ks[i];
            out.push(y);
            ct[i] = if encrypt { y } else { chunk[i] };
        }
        if n == 16 {
            *iv = ct;
        } else {
            iv.copy_within(n..16, 0);
            iv[16 - n..16].copy_from_slice(&ct[..n]);
        }
    }
    Ok(out)
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum AlgKind {
    Hash(Digest),
    Rsa,
    Ecdsa(Option<EcdsaCurve>),
    Aes,
    Rng,
}

#[repr(C)]
struct Alg {
    magic: u32,
    kind: AlgKind,
    hmac: bool,
    aes_mode: AesMode,
    msg_block_len: u32,
    ecc_curve: Option<EcdsaCurve>,
    name: String,
}

#[repr(C)]
struct Hash {
    magic: u32,
    digest: Digest,
    key: Option<Vec<u8>>,
    data: Vec<u8>,
}

enum KeyData {
    Rsa(RsaKey),
    PendingRsa {
        bits: u32,
    },
    Ecdsa(EcdsaKey),
    PendingEcdsa {
        curve: Option<EcdsaCurve>,
    },
    Aes {
        key: Vec<u8>,
        mode: AesMode,
        msg_block_len: u32,
    },
}

#[repr(C)]
struct Key {
    magic: u32,
    data: KeyData,
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

unsafe fn bytes<'a>(p: *const u8, n: u32) -> &'a [u8] {
    if p.is_null() || n == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(p, n as usize)
    }
}

fn aes_mode_to_u32(m: AesMode) -> u32 {
    match m {
        AesMode::Ecb => 0,
        AesMode::Cbc => 1,
        AesMode::Cfb => 2,
        AesMode::Ctr => 3,
        AesMode::Gcm => 4,
    }
}

fn aes_mode_from_u32(v: u32) -> Option<AesMode> {
    match v {
        0 => Some(AesMode::Ecb),
        1 => Some(AesMode::Cbc),
        2 => Some(AesMode::Cfb),
        3 => Some(AesMode::Ctr),
        4 => Some(AesMode::Gcm),
        _ => None,
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
    let name = crate::strings::from_wide(alg_id);
    let hmac = flags & BCRYPT_ALG_HANDLE_HMAC_FLAG != 0;
    let upper = name.to_ascii_uppercase();
    let kind = if let Some(d) = Digest::from_name(&name) {
        AlgKind::Hash(d)
    } else if matches!(upper.as_str(), "RSA" | "RSA_SIGN") && !hmac {
        AlgKind::Rsa
    } else if upper == "ECDSA_P256" && !hmac {
        AlgKind::Ecdsa(Some(EcdsaCurve::P256))
    } else if upper == "ECDSA_P384" && !hmac {
        AlgKind::Ecdsa(Some(EcdsaCurve::P384))
    } else if upper == "ECDSA_P521" && !hmac {
        AlgKind::Ecdsa(Some(EcdsaCurve::P521))
    } else if upper == "ECDSA" && !hmac {
        AlgKind::Ecdsa(None)
    } else if upper == "AES" && !hmac {
        AlgKind::Aes
    } else if upper == "RNG" && !hmac {
        AlgKind::Rng
    } else {
        stub!("BCryptOpenAlgorithmProvider({name:?}, flags 0x{flags:x}) -> STATUS_NOT_FOUND (algorithm not implemented)");
        return STATUS_NOT_FOUND;
    };
    let ecc_curve = match kind {
        AlgKind::Ecdsa(c) => c,
        _ => None,
    };
    log::debug!("BCryptOpenAlgorithmProvider({name:?}, flags 0x{flags:x}) -> {kind:?}");
    std::ptr::write_unaligned(
        out,
        Box::into_raw(Box::new(Alg {
            magic: ALG_MAGIC,
            kind,
            hmac,
            aes_mode: AesMode::Cbc,
            msg_block_len: 16,
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

fn wide_bytes(s: &str) -> Vec<u8> {
    let mut w: Vec<u8> = s.encode_utf16().flat_map(u16::to_le_bytes).collect();
    w.extend([0, 0]);
    w
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
        let value = match (p.as_str(), a.kind) {
            ("ObjectLength", _) => u32v(OBJECT_LENGTH),
            ("AlgorithmName", _) => wide_bytes(&a.name),
            ("HashDigestLength", AlgKind::Hash(d)) => u32v(d.len() as u32),
            ("HashBlockLength", AlgKind::Hash(d)) => u32v(d.block_len()),
            ("BlockLength", AlgKind::Aes) => u32v(16),
            ("BlockSizeList", AlgKind::Aes) => u32v(16),
            ("MessageBlockLength", AlgKind::Aes) => u32v(a.msg_block_len),
            ("ChainingMode", AlgKind::Aes) => wide_bytes(a.aes_mode.as_win_name()),
            ("KeyLengths", AlgKind::Aes) => key_lengths(128, 256, 64),
            ("AuthTagLength", AlgKind::Aes) => {
                // BCRYPT_AUTH_TAG_LENGTHS_STRUCT { dwMinLength=12, dwMaxLength=16, dwIncrement=1 }
                [12u32, 16, 1].into_iter().flat_map(u32::to_le_bytes).collect()
            }
            ("KeyLengths", AlgKind::Rsa) => key_lengths(512, 4096, 64),
            ("PaddingSchemes", AlgKind::Rsa) => u32v(BCRYPT_SUPPORTED_PAD_PKCS1_SIG | BCRYPT_SUPPORTED_PAD_PSS),
            ("KeyLengths", AlgKind::Ecdsa(Some(c))) => key_lengths(c.bits(), c.bits(), 0),
            ("KeyLengths", AlgKind::Ecdsa(None)) => match a.ecc_curve {
                Some(c) => key_lengths(c.bits(), c.bits(), 0),
                None => key_lengths(256, 521, 0),
            },
            ("ECCCurveName", AlgKind::Ecdsa(_)) => match a.ecc_curve {
                Some(c) => wide_bytes(c.win_curve_name()),
                None => return STATUS_NOT_FOUND,
            },
            _ => {
                stub!("BCryptGetProperty(alg={}, {p:?}) -> STATUS_NOT_SUPPORTED", a.name);
                return STATUS_NOT_SUPPORTED;
            }
        };
        return put_property(&value, out, out_len, result);
    }

    if let Some(o) = hash_obj(h) {
        let value = match p.as_str() {
            "ObjectLength" => u32v(OBJECT_LENGTH),
            "HashDigestLength" => u32v(o.digest.len() as u32),
            "HashBlockLength" => u32v(o.digest.block_len()),
            "AlgorithmName" => wide_bytes(o.digest.as_name()),
            _ => {
                stub!("BCryptGetProperty(hash, {p:?}) -> STATUS_NOT_SUPPORTED");
                return STATUS_NOT_SUPPORTED;
            }
        };
        return put_property(&value, out, out_len, result);
    }

    if let Some(k) = key(h) {
        let value = match (&k.data, p.as_str()) {
            (_, "ObjectLength") => u32v(OBJECT_LENGTH),
            (KeyData::Aes { .. }, "BlockLength") => u32v(16),
            (KeyData::Aes { .. }, "BlockSizeList") => u32v(16),
            (KeyData::Aes { key, .. }, "KeyLength" | "KeyStrength") => u32v((key.len() * 8) as u32),
            (KeyData::Aes { .. }, "KeyLengths") => key_lengths(128, 256, 64),
            (KeyData::Aes { mode, .. }, "ChainingMode") => wide_bytes(mode.as_win_name()),
            (KeyData::Aes { msg_block_len, .. }, "MessageBlockLength") => u32v(*msg_block_len),
            (KeyData::Aes { .. }, "AuthTagLength") => [12u32, 16, 1].into_iter().flat_map(u32::to_le_bytes).collect(),
            (KeyData::Aes { .. }, "AlgorithmName") => wide_bytes("AES"),
            (KeyData::Rsa(r), "KeyLength" | "KeyStrength") => u32v(r.pub_key.bits() as u32),
            (KeyData::Rsa(r), "BlockLength" | "SignatureLength") => u32v(r.pub_key.k() as u32),
            (KeyData::Rsa(_), "KeyLengths") => key_lengths(512, 4096, 64),
            (KeyData::Rsa(_), "AlgorithmName") => wide_bytes("RSA"),
            (KeyData::PendingRsa { bits }, "KeyLength" | "KeyStrength") => u32v(*bits),
            (KeyData::PendingRsa { bits }, "BlockLength" | "SignatureLength") => u32v((*bits as usize).div_ceil(8) as u32),
            (KeyData::PendingRsa { .. }, "AlgorithmName") => wide_bytes("RSA"),
            (KeyData::Ecdsa(e), "KeyLength" | "KeyStrength") => u32v(e.curve.bits()),
            (KeyData::Ecdsa(e), "SignatureLength") => u32v((e.curve.coord_len() * 2) as u32),
            (KeyData::Ecdsa(e), "KeyLengths") => key_lengths(e.curve.bits(), e.curve.bits(), 0),
            (KeyData::Ecdsa(e), "ECCCurveName") => wide_bytes(e.curve.win_curve_name()),
            (KeyData::Ecdsa(e), "AlgorithmName") => wide_bytes(e.curve.alg_name()),
            (KeyData::PendingEcdsa { curve: Some(c) }, "KeyLength" | "KeyStrength") => u32v(c.bits()),
            (KeyData::PendingEcdsa { curve: Some(c) }, "SignatureLength") => u32v((c.coord_len() * 2) as u32),
            (KeyData::PendingEcdsa { curve: Some(c) }, "ECCCurveName") => wide_bytes(c.win_curve_name()),
            (KeyData::PendingEcdsa { curve: Some(c) }, "AlgorithmName") => wide_bytes(c.alg_name()),
            _ => {
                stub!("BCryptGetProperty(key, {p:?}) -> STATUS_NOT_SUPPORTED");
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
    _flags: u32,
) -> i32 {
    let Some(a) = alg(h) else { return STATUS_INVALID_HANDLE };
    let AlgKind::Hash(digest) = a.kind else { return STATUS_INVALID_HANDLE };
    if out.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let key = a.hmac.then(|| bytes(secret, secret_len).to_vec());
    std::ptr::write_unaligned(
        out,
        Box::into_raw(Box::new(Hash { magic: HASH_MAGIC, digest, key, data: Vec::new() })) as *mut c_void,
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
    if out.is_null() || out_len as usize != o.digest.len() {
        return STATUS_INVALID_PARAMETER;
    }
    let d = match &o.key {
        Some(k) => o.digest.hmac(k, &o.data),
        None => o.digest.hash(&o.data),
    };
    std::ptr::copy_nonoverlapping(d.as_ptr(), out, d.len());
    // A finished hash object cannot be reused; reset it so misuse is visible.
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
        stub!("BCryptImportKeyPair({}, wrapped import_key) -> STATUS_NOT_SUPPORTED", a.name);
        return STATUS_NOT_SUPPORTED;
    }
    let t = crate::strings::from_wide(blob_type);
    let blob = bytes(input, input_len);
    match a.kind {
        AlgKind::Rsa => {
            if !matches!(t.as_str(), "RSAPUBLICBLOB" | "RSAPRIVATEBLOB" | "RSAFULLPRIVATEBLOB" | "PUBLICBLOB" | "PRIVATEBLOB") {
                stub!("BCryptImportKeyPair({}, {t:?}) -> STATUS_NOT_SUPPORTED", a.name);
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
        AlgKind::Ecdsa(alg_curve) => {
            if !matches!(t.as_str(), "ECCPUBLICBLOB" | "ECCPRIVATEBLOB" | "PUBLICBLOB" | "PRIVATEBLOB") {
                stub!("BCryptImportKeyPair({}, {t:?}) -> STATUS_NOT_SUPPORTED", a.name);
                return STATUS_NOT_SUPPORTED;
            }
            let expected = alg_curve.or(a.ecc_curve);
            let Some(ec) = EcdsaKey::from_blob(&t, expected, blob) else {
                return STATUS_INVALID_PARAMETER;
            };
            log::debug!("BCryptImportKeyPair({t}): {:?} ECDSA key (priv={})", ec.curve, ec.d.is_some());
            std::ptr::write_unaligned(
                out,
                Box::into_raw(Box::new(Key { magic: KEY_MAGIC, data: KeyData::Ecdsa(ec) })) as *mut c_void,
            );
            STATUS_SUCCESS
        }
        _ => {
            stub!("BCryptImportKeyPair({}, {t:?}) -> STATUS_NOT_SUPPORTED", a.name);
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

unsafe fn parse_rsa_padding(padding: *mut c_void, flags: u32, hash_len: usize) -> Result<(Option<Digest>, Option<usize>), i32> {
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
    if (flags & BCRYPT_PAD_PSS) != 0 {
        let Some(_) = digest else { return Err(STATUS_INVALID_PARAMETER) };
        if padding.is_null() {
            return Err(STATUS_INVALID_PARAMETER);
        }
        let salt = std::ptr::read_unaligned((padding as *const u8).add(8) as *const u32) as usize;
        Ok((digest, Some(salt)))
    } else if (flags & BCRYPT_PAD_PKCS1) != 0 {
        Ok((digest, None))
    } else {
        Err(STATUS_NOT_SUPPORTED)
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
            if rsa.priv_params.is_none() {
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
            let (digest, pss_salt) = match parse_rsa_padding(padding, flags, hash.len()) {
                Ok(v) => v,
                Err(STATUS_NOT_SUPPORTED) => {
                    stub!("BCryptSignHash(RSA, flags 0x{flags:x}) -> STATUS_NOT_SUPPORTED");
                    return STATUS_NOT_SUPPORTED;
                }
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
        KeyData::Ecdsa(ec) => {
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
            let (digest, pss_salt) = match parse_rsa_padding(padding, flags, hash_b.len()) {
                Ok(v) => v,
                Err(STATUS_NOT_SUPPORTED) => {
                    stub!("BCryptVerifySignature(RSA, flags 0x{flags:x}) -> STATUS_NOT_SUPPORTED (padding)");
                    return STATUS_NOT_SUPPORTED;
                }
                Err(e) => return e,
            };
            if sig_b.len() != rsa.pub_key.k() {
                return STATUS_INVALID_BUFFER_SIZE;
            }
            let ok = match pss_salt {
                Some(salt) => rsa.pub_key.verify_pss(digest.unwrap(), hash_b, salt, sig_b),
                None => rsa.pub_key.verify_pkcs1(digest, hash_b, sig_b),
            };
            log::debug!("BCryptVerifySignature(RSA, flags 0x{flags:x}) -> {}", if ok { "valid" } else { "INVALID" });
            if ok { STATUS_SUCCESS } else { STATUS_INVALID_SIGNATURE }
        }
        KeyData::Ecdsa(ec) => {
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

#[allow(clippy::too_many_arguments)]
unsafe fn bcrypt_aes_crypt(
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
    let Some(k) = key(h_key) else {
        return STATUS_INVALID_HANDLE;
    };
    let KeyData::Aes { ref key, mode, msg_block_len } = k.data else {
        stub!("BCrypt{} on non-AES key -> STATUS_NOT_SUPPORTED", if encrypt { "Encrypt" } else { "Decrypt" });
        return STATUS_NOT_SUPPORTED;
    };
    if result_len.is_null() || (input_len > 0 && input.is_null()) {
        return STATUS_INVALID_PARAMETER;
    }

    let in_slice = bytes(input, input_len);
    let pkcs7 = (flags & BCRYPT_BLOCK_PADDING) != 0;

    // Size-query path (output == NULL): return exact required output buffer size without mutating IV/tag.
    if output.is_null() {
        let need = match mode {
            AesMode::Ecb | AesMode::Cbc => {
                if encrypt && pkcs7 {
                    ((in_slice.len() / 16) + 1) * 16
                } else {
                    if in_slice.len() % 16 != 0 {
                        return STATUS_INVALID_BUFFER_SIZE;
                    }
                    in_slice.len()
                }
            }
            AesMode::Cfb | AesMode::Ctr | AesMode::Gcm => in_slice.len(),
        };
        std::ptr::write_unaligned(result_len, need as u32);
        return STATUS_SUCCESS;
    }

    let out_bytes = match mode {
        AesMode::Ecb => match aes_cccrypt(encrypt, true, pkcs7, key, None, in_slice) {
            Ok(v) => v,
            Err(e) => return e,
        },
        AesMode::Cbc => {
            let mut iv_blk = [0u8; 16];
            if !iv.is_null() {
                if iv_len < 16 {
                    return STATUS_INVALID_PARAMETER;
                }
                std::ptr::copy_nonoverlapping(iv, iv_blk.as_mut_ptr(), 16);
            }
            // Save last ciphertext block before decryption in case input == output.
            let next_iv_dec = if !encrypt && in_slice.len() >= 16 {
                let mut b = [0u8; 16];
                b.copy_from_slice(&in_slice[in_slice.len() - 16..]);
                Some(b)
            } else {
                None
            };
            let v = match aes_cccrypt(encrypt, false, pkcs7, key, Some(&iv_blk), in_slice) {
                Ok(v) => v,
                Err(e) => return e,
            };
            if (output_len as usize) < v.len() {
                std::ptr::write_unaligned(result_len, v.len() as u32);
                return STATUS_BUFFER_TOO_SMALL;
            }
            if !iv.is_null() {
                if encrypt && v.len() >= 16 {
                    std::ptr::copy_nonoverlapping(v[v.len() - 16..].as_ptr(), iv, 16);
                } else if let Some(b) = next_iv_dec {
                    std::ptr::copy_nonoverlapping(b.as_ptr(), iv, 16);
                }
            }
            v
        }
        AesMode::Ctr => {
            let mut iv_blk = [0u8; 16];
            if !iv.is_null() {
                if iv_len < 16 {
                    return STATUS_INVALID_PARAMETER;
                }
                std::ptr::copy_nonoverlapping(iv, iv_blk.as_mut_ptr(), 16);
            }
            let v = match aes_ctr_crypt(key, &mut iv_blk, in_slice) {
                Ok(v) => v,
                Err(e) => return e,
            };
            if (output_len as usize) < v.len() {
                std::ptr::write_unaligned(result_len, v.len() as u32);
                return STATUS_BUFFER_TOO_SMALL;
            }
            if !iv.is_null() {
                std::ptr::copy_nonoverlapping(iv_blk.as_ptr(), iv, 16);
            }
            v
        }
        AesMode::Cfb => {
            let mut iv_blk = [0u8; 16];
            if !iv.is_null() {
                if iv_len < 16 {
                    return STATUS_INVALID_PARAMETER;
                }
                std::ptr::copy_nonoverlapping(iv, iv_blk.as_mut_ptr(), 16);
            }
            let v = match aes_cfb_crypt(encrypt, key, &mut iv_blk, msg_block_len as usize, in_slice) {
                Ok(v) => v,
                Err(e) => return e,
            };
            if (output_len as usize) < v.len() {
                std::ptr::write_unaligned(result_len, v.len() as u32);
                return STATUS_BUFFER_TOO_SMALL;
            }
            if !iv.is_null() {
                std::ptr::copy_nonoverlapping(iv_blk.as_ptr(), iv, 16);
            }
            v
        }
        AesMode::Gcm => {
            if padding_info.is_null() {
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
            let (v, tag) = match aes_gcm_crypt(encrypt, key, nonce, aad, in_slice, tag_len, expected_tag) {
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
    bcrypt_aes_crypt(false, h_key, input, input_len, padding_info, iv, iv_len, output, output_len, result_len, flags)
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
    bcrypt_aes_crypt(true, h_key, input, input_len, padding_info, iv, iv_len, output, output_len, result_len, flags)
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
    if !h_import_key.is_null() {
        stub!("BCryptImportKey({}, wrapped h_import_key) -> STATUS_NOT_SUPPORTED", a.name);
        return STATUS_NOT_SUPPORTED;
    }
    let t = crate::strings::from_wide(blob_type);
    let blob = bytes(input, input_len);
    if a.kind == AlgKind::Aes && t == "KeyDataBlob" {
        if blob.len() < 12 {
            return STATUS_INVALID_PARAMETER;
        }
        let magic = u32::from_le_bytes(blob[0..4].try_into().unwrap());
        let version = u32::from_le_bytes(blob[4..8].try_into().unwrap());
        let cb_key = u32::from_le_bytes(blob[8..12].try_into().unwrap()) as usize;
        if magic != BCRYPT_KEY_DATA_BLOB_MAGIC || version != 1 || blob.len() < 12 + cb_key || !matches!(cb_key, 16 | 24 | 32) {
            return STATUS_INVALID_PARAMETER;
        }
        let k = Key {
            magic: KEY_MAGIC,
            data: KeyData::Aes {
                key: blob[12..12 + cb_key].to_vec(),
                mode: a.aes_mode,
                msg_block_len: a.msg_block_len,
            },
        };
        std::ptr::write_unaligned(ph_key, Box::into_raw(Box::new(k)) as *mut c_void);
        return STATUS_SUCCESS;
    }
    if a.kind == AlgKind::Aes && t == "OpaqueKeyBlob" {
        if blob.len() < 20 {
            return STATUS_INVALID_PARAMETER;
        }
        let magic = u32::from_le_bytes(blob[0..4].try_into().unwrap());
        let version = u32::from_le_bytes(blob[4..8].try_into().unwrap());
        let mode_raw = u32::from_le_bytes(blob[8..12].try_into().unwrap());
        let mbl = u32::from_le_bytes(blob[12..16].try_into().unwrap());
        let cb_key = u32::from_le_bytes(blob[16..20].try_into().unwrap()) as usize;
        let Some(mode) = aes_mode_from_u32(mode_raw) else {
            return STATUS_INVALID_PARAMETER;
        };
        if magic != BCRYPT_OPAQUE_KEY_MAGIC
            || version != 1
            || !(1..=16).contains(&mbl)
            || blob.len() < 20 + cb_key
            || !matches!(cb_key, 16 | 24 | 32)
        {
            return STATUS_INVALID_PARAMETER;
        }
        let k = Key {
            magic: KEY_MAGIC,
            data: KeyData::Aes {
                key: blob[20..20 + cb_key].to_vec(),
                mode,
                msg_block_len: mbl,
            },
        };
        std::ptr::write_unaligned(ph_key, Box::into_raw(Box::new(k)) as *mut c_void);
        return STATUS_SUCCESS;
    }
    stub!("BCryptImportKey({}, {t:?}) -> STATUS_NOT_SUPPORTED", a.name);
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
    if !h_export_key.is_null() {
        std::ptr::write_unaligned(result_len, 0);
        stub!("BCryptExportKey(wrapped h_export_key) -> STATUS_NOT_SUPPORTED");
        return STATUS_NOT_SUPPORTED;
    }
    let Some(k) = key(h_key) else {
        return STATUS_INVALID_HANDLE;
    };
    let t = crate::strings::from_wide(blob_type);
    match (&k.data, t.as_str()) {
        (KeyData::Aes { key, .. }, "KeyDataBlob") => {
            let mut blob = Vec::with_capacity(12 + key.len());
            blob.extend_from_slice(&BCRYPT_KEY_DATA_BLOB_MAGIC.to_le_bytes());
            blob.extend_from_slice(&1u32.to_le_bytes());
            blob.extend_from_slice(&(key.len() as u32).to_le_bytes());
            blob.extend_from_slice(key);
            return put_property(&blob, output, output_len, result_len);
        }
        (KeyData::Aes { key, mode, msg_block_len }, "OpaqueKeyBlob") => {
            let mut blob = Vec::with_capacity(20 + key.len());
            blob.extend_from_slice(&BCRYPT_OPAQUE_KEY_MAGIC.to_le_bytes());
            blob.extend_from_slice(&1u32.to_le_bytes());
            blob.extend_from_slice(&aes_mode_to_u32(*mode).to_le_bytes());
            blob.extend_from_slice(&msg_block_len.to_le_bytes());
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
        (KeyData::Ecdsa(ec), "ECCPUBLICBLOB" | "PUBLICBLOB") => {
            let blob = ec.to_public_blob();
            return put_property(&blob, output, output_len, result_len);
        }
        (KeyData::Ecdsa(ec), "ECCPRIVATEBLOB" | "PRIVATEBLOB") => {
            let Some(blob) = ec.to_private_blob() else {
                return STATUS_NOT_SUPPORTED;
            };
            return put_property(&blob, output, output_len, result_len);
        }
        (KeyData::PendingRsa { .. } | KeyData::PendingEcdsa { .. }, _) => {
            return STATUS_INVALID_HANDLE;
        }
        _ => {}
    }
    std::ptr::write_unaligned(result_len, 0);
    stub!("BCryptExportKey({t:?}) -> STATUS_NOT_SUPPORTED");
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
    if a.kind != AlgKind::Aes {
        stub!("BCryptGenerateSymmetricKey({}) -> STATUS_NOT_SUPPORTED", a.name);
        return STATUS_NOT_SUPPORTED;
    }
    if !matches!(secret_len, 16 | 24 | 32) {
        return STATUS_INVALID_PARAMETER;
    }
    let k = Key {
        magic: KEY_MAGIC,
        data: KeyData::Aes {
            key: bytes(secret, secret_len).to_vec(),
            mode: a.aes_mode,
            msg_block_len: a.msg_block_len,
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
        KeyData::Ecdsa(e) => KeyData::Ecdsa(e.clone()),
        KeyData::PendingEcdsa { curve } => KeyData::PendingEcdsa { curve: *curve },
        KeyData::Aes { key, mode, msg_block_len } => KeyData::Aes {
            key: key.clone(),
            mode: *mode,
            msg_block_len: *msg_block_len,
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
        AlgKind::Ecdsa(Some(curve)) => {
            if length != 0 && length != curve.bits() {
                return STATUS_INVALID_PARAMETER;
            }
            KeyData::PendingEcdsa { curve: Some(curve) }
        }
        AlgKind::Ecdsa(None) => {
            let curve = if length != 0 {
                let Some(c) = EcdsaCurve::from_bits(length) else {
                    return STATUS_INVALID_PARAMETER;
                };
                Some(c)
            } else {
                a.ecc_curve
            };
            KeyData::PendingEcdsa { curve }
        }
        _ => {
            stub!("BCryptGenerateKeyPair({}) -> STATUS_NOT_SUPPORTED", a.name);
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
        KeyData::PendingEcdsa { curve } => {
            let Some(c) = curve else {
                return STATUS_INVALID_PARAMETER;
            };
            match EcdsaKey::generate(c) {
                Ok(ec) => {
                    k.data = KeyData::Ecdsa(ec);
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
        let Some(mode) = AesMode::from_name(&mode_str) else {
            stub!("BCryptSetProperty(ChainingMode={mode_str:?}) -> STATUS_NOT_SUPPORTED");
            return STATUS_NOT_SUPPORTED;
        };
        if let Some(a) = alg_mut(h_obj) {
            if a.kind != AlgKind::Aes {
                return STATUS_NOT_SUPPORTED;
            }
            a.aes_mode = mode;
            return STATUS_SUCCESS;
        }
        if let Some(k) = key_mut(h_obj) {
            if let KeyData::Aes { mode: ref mut km, .. } = k.data {
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
        if !(1..=16).contains(&mbl) {
            return STATUS_INVALID_PARAMETER;
        }
        if let Some(a) = alg_mut(h_obj) {
            a.msg_block_len = mbl;
            return STATUS_SUCCESS;
        }
        if let Some(k) = key_mut(h_obj) {
            if let KeyData::Aes { msg_block_len: ref mut km, .. } = k.data {
                *km = mbl;
                return STATUS_SUCCESS;
            }
        }
        return STATUS_INVALID_HANDLE;
    }

    if p == "ECCCurveName" {
        if in_bytes.len() < 2 || in_bytes.len() % 2 != 0 {
            return STATUS_INVALID_PARAMETER;
        }
        let units: Vec<u16> = in_bytes.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
        let curve_str = String::from_utf16_lossy(&units);
        let Some(curve) = EcdsaCurve::from_win_curve_name(&curve_str) else {
            stub!("BCryptSetProperty(ECCCurveName={curve_str:?}) -> STATUS_NOT_SUPPORTED");
            return STATUS_NOT_SUPPORTED;
        };
        if let Some(a) = alg_mut(h_obj) {
            if matches!(a.kind, AlgKind::Ecdsa(None)) {
                a.ecc_curve = Some(curve);
                return STATUS_SUCCESS;
            }
            return STATUS_NOT_SUPPORTED;
        }
        if let Some(k) = key_mut(h_obj) {
            if let KeyData::PendingEcdsa { curve: ref mut kc } = k.data {
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
            if let KeyData::PendingRsa { bits: ref mut kb } = k.data {
                if !(1024..=4096).contains(&bits) || bits % 64 != 0 {
                    return STATUS_INVALID_PARAMETER;
                }
                *kb = bits;
                return STATUS_SUCCESS;
            }
            return STATUS_NOT_SUPPORTED;
        }
        return STATUS_INVALID_HANDLE;
    }

    stub!("BCryptSetProperty({p:?}) -> STATUS_NOT_SUPPORTED");
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
        digest: o.digest,
        key: o.key.clone(),
        data: o.data.clone(),
    };
    std::ptr::write_unaligned(ph_new, Box::into_raw(Box::new(dup)) as *mut c_void);
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

        // Oversized or nonsensical RSA parameters must be rejected immediately before bn_modexp.
        let mut huge_e = vec![0xFFu8; 65_536];
        huge_e[0] = 0x7F; // positive DER integer, odd, 64 KiB
        assert!(RsaPublicKey::from_asn1_integers(&k.n, &huge_e).is_none());
        let huge_e_key = RsaPublicKey {
            n: k.n.clone(),
            e: huge_e,
        };
        assert!(!huge_e_key.verify_pkcs1(Some(Digest::Sha256), &h, &hex(SIG_PKCS1)));

        let mut huge_n = vec![0xFFu8; RsaPublicKey::MAX_MODULUS_BYTES + 1];
        huge_n[0] = 0x7F;
        assert!(RsaPublicKey::from_asn1_integers(&huge_n, &[0x01, 0x00, 0x01]).is_none());
        assert!(RsaPublicKey::from_asn1_integers(&k.n, &[0x01]).is_none()); // e == 1
        assert!(RsaPublicKey::from_asn1_integers(&k.n, &[0x02]).is_none()); // even e
        assert!(RsaPublicKey::from_asn1_integers(&[0x80; 256], &[0x01, 0x00, 0x01]).is_none()); // negative DER
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
}
