//! AK's sound-engine random generator [V].
//!
//! One global 64-bit LCG at `0x1439310e8`: `state = state * 0x5851f42d4c957f2d + 1`, the value
//! used is `state >> 33` (31 bits). Every draw site inlines the step (e.g. `FUN_1415cd350`,
//! `FUN_141540520`, `FUN_1415c0040`); the plugin-context vfunc `0xd8` (`FUN_141585eb0`) exposes
//! it to plugins. `SoundEngine::SetRandomSeed(u32)` (`0x141589b90`) queues command `0x34`; the
//! audio thread applies it with `mov ecx, dword [r13+4]; call 0x14157f120` (`0x1415a9819`), and
//! `0x14157f120` stores `rcx` to the state, i.e. the seed zero-extended to 64 bits.
//!
//! Draw shapes (all [V], from the decompile):
//! * ranged property modifier (`FUN_1415c0040`, `FUN_1415c0d60`, `FUN_1415fbfc0`): no draw when
//!   `max - min == 0`, else `(float)r * 2^-31 * (max - min)` in f32, then `+ min`;
//! * RanSeq loop-count modifier (`FUN_1415f9720`): `(short)(int)((double)r * 2^-31 *
//!   (double)(max - min) + 0.5)`, no draw when `max - min == 0`;
//! * container index: `r % n` (u32 division; `FUN_1415fb3a0`, `FUN_1415cd350`).

/// `2^-31` as f32 (`0x30000000`, printed by Ghidra as `4.656613e-10`).
pub const INV_2_31_F32: f32 = 1.0 / 2_147_483_648.0;
/// `2^-31` as f64 (`4.656612875245797e-10`).
pub const INV_2_31_F64: f64 = 1.0 / 2_147_483_648.0;

#[derive(Debug, Clone)]
pub struct AkRng {
    pub state: u64,
    /// Number of draws taken (diagnostics).
    pub draws: u64,
}

impl AkRng {
    /// State after `SetRandomSeed(seed)`.
    pub fn new(seed: u32) -> Self {
        AkRng { state: seed as u64, draws: 0 }
    }

    /// One LCG step; returns `state >> 33`.
    pub fn next(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(0x5851_f42d_4c95_7f2d).wrapping_add(1);
        self.draws += 1;
        (self.state >> 33) as u32
    }

    /// Ranged-modifier offset (without `min`): 0 without a draw when the range is empty.
    pub fn ranged_offset(&mut self, min: f32, max: f32) -> f32 {
        let d = max - min;
        if d == 0.0 {
            return 0.0;
        }
        self.next() as f32 * INV_2_31_F32 * d
    }

    /// RanSeq loop-count offset (`FUN_1415f9720`): `range = max - min` (shorts).
    pub fn loop_offset(&mut self, range: i16) -> i16 {
        if range == 0 {
            return 0;
        }
        (self.next() as f64 * INV_2_31_F64 * range as f64 + 0.5) as i32 as i16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_are_powers_of_two() {
        assert_eq!(INV_2_31_F32.to_bits(), 0x3000_0000);
        assert_eq!(INV_2_31_F64, 2f64.powi(-31));
    }

    #[test]
    fn lcg_sequence() {
        let mut r = AkRng::new(0x5eed);
        let s1 = 0x5eedu64.wrapping_mul(0x5851_f42d_4c95_7f2d).wrapping_add(1);
        assert_eq!(r.next(), (s1 >> 33) as u32);
        let s2 = s1.wrapping_mul(0x5851_f42d_4c95_7f2d).wrapping_add(1);
        assert_eq!(r.next(), (s2 >> 33) as u32);
        assert_eq!(r.ranged_offset(1.0, 1.0), 0.0);
        assert_eq!(r.draws, 2);
    }
}
