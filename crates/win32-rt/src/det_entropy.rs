//! Deterministic entropy for the dll_lift / exe_lift difftest children.
//!
//! `rand_s`, `BCryptGenRandom`, `SystemFunction036` (`RtlGenRandom`) and `CryptGenRandom`
//! normally return host randomness. A function that seeds a generator from them (V8's
//! `base::RandomNumberGenerator`, created on demand by `Isolate::random_number_generator`)
//! is then not a function of the inputs the difftest restores, and its guest and lifted runs
//! differ by chance. A difftest child calls [`reset`] before every run: from then on those
//! APIs return a splitmix64 stream that restarts at the same seed for the guest and the lifted
//! run of a trial. Off (0) everywhere else, including gameplay.
use core::sync::atomic::{AtomicU64, Ordering::Relaxed};

static STATE: AtomicU64 = AtomicU64::new(0);

/// Restart the stream at `seed` (non-zero: on; 0: host randomness again).
pub fn reset(seed: u64) {
    STATE.store(seed, Relaxed);
}

/// Fill `len` bytes at `buf` from the stream if it is on; `false` if it is off.
///
/// # Safety
/// `buf` must be valid for `len` bytes of writes.
pub unsafe fn fill(buf: *mut u8, len: usize) -> bool {
    if STATE.load(Relaxed) == 0 {
        return false;
    }
    let mut i = 0;
    while i < len {
        let v = next().to_le_bytes();
        let n = (len - i).min(8);
        core::ptr::copy_nonoverlapping(v.as_ptr(), buf.add(i), n);
        i += n;
    }
    true
}

fn next() -> u64 {
    // splitmix64; the state never returns to 0 in practice (period 2^64 over all values).
    let s = STATE.fetch_add(0x9E37_79B9_7F4A_7C15, Relaxed).wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = s;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

#[cfg(test)]
mod tests {
    #[test]
    fn stream_restarts_at_the_seed() {
        let (mut a, mut b) = ([0u8; 13], [0u8; 13]);
        super::reset(7);
        assert!(unsafe { super::fill(a.as_mut_ptr(), a.len()) });
        super::reset(7);
        assert!(unsafe { super::fill(b.as_mut_ptr(), b.len()) });
        assert_eq!(a, b);
        super::reset(0);
        assert!(!unsafe { super::fill(a.as_mut_ptr(), a.len()) });
    }
}
