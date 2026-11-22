//! `wwise-audio` — native audio for the Alan Wake 2 macOS port.
//!
//! Alan Wake 2 statically links Audiokinetic Wwise 2022.1.4 (`AK::` in AlanWake2.exe). This
//! crate is the foundation of an open replacement (see docs/AUDIO.md):
//!
//! * [`bnk`] — SoundBank parser for bank generator version 145 (Wwise 2022.1): BKHD, DIDX,
//!   DATA, HIRC (typed objects), STID, STMG, ENVS, INIT, PLAT.
//! * [`wem`] — Wwise RIFF media container.
//! * [`codec`] — decoders for the codecs present in the game data (Wwise Vorbis).
//! * [`data`] — access to `data_pc/audio/**` through the Northlight VFS (aw2-data).
//!
//! Format knowledge comes from the vendored reference implementations `third_party/wwiser`
//! (bank layout, v145 branches) and `third_party/vgmstream` (.wem/Vorbis), and every parser is
//! validated against the full game data set (`wwise-audio inventory`) and vgmstream output.

pub mod bnk;
pub mod codec;
pub mod data;
pub mod engine;
pub mod reader;
pub mod runtime;
pub mod wav;
pub mod wem;


/// Wwise short-ID hash: 32-bit FNV-1 over the lower-cased name (AK `AK::SoundEngine::GetIDFromString`).
pub fn fnv1_32_lower(name: &str) -> u32 {
    let mut h: u32 = 2166136261;
    for b in name.bytes() {
        h = h.wrapping_mul(16777619);
        h ^= b.to_ascii_lowercase() as u32;
    }
    h
}

#[cfg(test)]
mod tests {
    #[test]
    fn fnv_matches_known_bank_ids() {
        // Init.bnk BKHD dwSoundBankID = 1355168291 (wwiser dump labels it "Init").
        assert_eq!(super::fnv1_32_lower("Init"), 1355168291);
    }
}
