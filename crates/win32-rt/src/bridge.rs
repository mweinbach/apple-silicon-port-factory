//! aarch64 host-bridge argument adapters for natively implemented Win64 functions
//! that take float arguments by value.
//!
//! On aarch64 the interpreter calls host functions through
//! `aw2_game::emu::call_host_with_xmm`, which passes arguments by position:
//! Win64 slot k goes in x(k-1) for k <= 8, slots 9..16 go to `[sp + 8*(k-9)]`
//! (read from the guest stack), and xmm0..3 go in v0..v3. The callee's types play
//! no part. AAPCS64 assigns integer and float parameters to separate register files.
//! So a plain Rust signature that mixes them, such as `fn(id: u32, v: f32, obj: u64)`,
//! reads `obj` from x1 (the guest's rdx, which held nothing for the float slot) and
//! `v` from v0 (guest xmm0) instead of v1. All-integer and all-float signatures
//! are unaffected.
//!
//! [`win64_bridge!`](crate::win64_bridge) generates, next to such a function
//! `f`, a module `f` holding `bridge`: an aarch64 entry that takes every slot
//! raw (16 integer slots + 4 xmm registers) and rebuilds the typed arguments
//! via [`FromSlot`]. Float slots 1..4 come from xmm, float slots 5+ from the
//! stack slot bits, and everything else from the integer slot. Register
//! `bridge_entry!(f)` (which is just `f` off aarch64) wherever the guest calls it.

/// Converts one Win64 argument slot into a typed argument. `int` is the integer
/// register / stack value of the slot. `xmm` is the low 64 bits of xmm(k-1)
/// for slots 1..4, and the same as `int` for stack slots.
pub trait FromSlot {
    fn from_slot(int: u64, xmm: u64) -> Self;
}

macro_rules! from_int {
    ($($t:ty),*) => { $(impl FromSlot for $t {
        #[inline(always)]
        fn from_slot(int: u64, _xmm: u64) -> Self { int as $t }
    })* };
}
from_int!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl FromSlot for bool {
    /// MSVC passes `bool` in the low byte; the rest of the slot is unspecified.
    #[inline(always)]
    fn from_slot(int: u64, _xmm: u64) -> Self {
        int as u8 != 0
    }
}
impl<T> FromSlot for *const T {
    #[inline(always)]
    fn from_slot(int: u64, _xmm: u64) -> Self {
        int as usize as *const T
    }
}
impl<T> FromSlot for *mut T {
    #[inline(always)]
    fn from_slot(int: u64, _xmm: u64) -> Self {
        int as usize as *mut T
    }
}
impl FromSlot for f32 {
    #[inline(always)]
    fn from_slot(_int: u64, xmm: u64) -> Self {
        f32::from_bits(xmm as u32)
    }
}
impl FromSlot for f64 {
    #[inline(always)]
    fn from_slot(_int: u64, xmm: u64) -> Self {
        f64::from_bits(xmm)
    }
}

/// Positional cursor over the raw slots handed to a bridge entry.
pub struct Slots {
    int: [u64; 16],
    xmm: [u64; 4],
    k: usize,
}

impl Slots {
    #[inline(always)]
    pub fn new(int: [u64; 16], xmm: [u64; 4]) -> Self {
        Slots { int, xmm, k: 0 }
    }
    /// The next slot as `T`.
    #[inline(always)]
    pub fn next<T: FromSlot>(&mut self) -> T {
        let k = self.k;
        self.k += 1;
        let int = self.int[k];
        T::from_slot(int, if k < 4 { self.xmm[k] } else { int })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Slot k mapping: ints from the integer slot, floats from xmm(k-1) for
    /// k <= 4 and from the stack-slot bits after that.
    #[test]
    fn slots_follow_win64_positions() {
        let mut int = [0u64; 16];
        for (i, v) in int.iter_mut().enumerate() {
            *v = 0x1000 + i as u64;
        }
        int[5] = 2.5f32.to_bits() as u64 | 0xdead_0000_0000; // float in a stack slot
        let xmm = [1.0f64.to_bits(), 0.5f32.to_bits() as u64, 0, 0];
        let mut s = Slots::new(int, xmm);
        assert_eq!(s.next::<u32>(), 0x1000); // slot 1: int
        assert_eq!(s.next::<f32>(), 0.5); // slot 2: xmm1
        assert_eq!(s.next::<u64>(), 0x1002); // slot 3: r8
        assert_eq!(s.next::<i32>(), 0x1003); // slot 4: r9
        assert_eq!(s.next::<bool>(), true); // slot 5: low byte 0x04
        assert_eq!(s.next::<f32>(), 2.5); // slot 6: float from the stack slot
    }
}
