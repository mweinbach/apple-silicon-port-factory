//! Channel layouts and 2D "direct speaker assignment" matrices (ADVICE_A3 §1, §4).
//!
//! `AkChannelConfig` = `uNumChannels:8 | eConfigType:4 | uChannelMask:20`; config types 0
//! anonymous, 1 standard, 2 ambisonic. Buffer order: speakers in mask-bit order with **LFE last**.
//! Matrices are `[in_channel][out_channel]`.

/// Speaker bits.
pub const FL: u32 = 0x1;
pub const FR: u32 = 0x2;
pub const FC: u32 = 0x4;
pub const LFE: u32 = 0x8;

const H: f32 = 0.707_106_77; // 0x3f3504f3

/// Downmix tables `float[18][n]` from AlanWake2.exe, indexed by input speaker bit position,
/// columns in output order [V: read from the exe]: mono @0x14268c580, stereo @0x14268c5e0.
const TABLE_MONO: [f32; 18] = [H, H, 1.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.5, H, 0.5, 0.353_553_4, 0.0, 0.353_553_4];
const TABLE_STEREO: [[f32; 2]; 18] = [
    [1.0, 0.0],
    [0.0, 1.0],
    [H, H],
    [0.0, 0.0],
    [H, 0.0],
    [0.0, H],
    [0.0, 0.0],
    [0.0, 0.0],
    [H, H],
    [H, 0.0],
    [0.0, H],
    [H, H],
    [H, 0.0],
    [0.5, 0.5],
    [0.0, H],
    [0.5, 0.0],
    [0.0, 0.0],
    [0.0, 0.5],
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Layout {
    pub channels: usize,
    /// 0 anonymous, 1 standard, 2 ambisonic.
    pub kind: u8,
    pub mask: u32,
}

impl Layout {
    pub const STEREO: Layout = Layout { channels: 2, kind: 1, mask: FL | FR };
    pub const MONO: Layout = Layout { channels: 1, kind: 1, mask: FC };

    /// From an `AkChannelConfig` word (bus `channel_config`, `.wem` fmt+0x14); 0 = unset.
    pub fn from_config(cfg: u32) -> Option<Layout> {
        if cfg == 0 {
            return None;
        }
        Some(Layout { channels: (cfg & 0xff) as usize, kind: ((cfg >> 8) & 0xf) as u8, mask: cfg >> 12 })
    }

    pub fn from_wem(channels: u16, kind: u8, mask: u32) -> Layout {
        let mask = if kind == 1 || (kind == 0 && mask != 0 && mask.count_ones() as u16 == channels) {
            mask
        } else if channels == 1 {
            FC
        } else if channels == 2 {
            FL | FR
        } else {
            0
        };
        Layout { channels: channels as usize, kind: if mask != 0 { 1 } else { kind }, mask }
    }

    /// Speaker bit index of each buffer channel (LFE last). Empty for non-standard layouts.
    pub fn speakers(&self) -> Vec<u32> {
        if self.kind != 1 || self.mask.count_ones() as usize != self.channels {
            return vec![];
        }
        let mut v: Vec<u32> = (0..20).filter(|b| self.mask & (1 << b) != 0 && *b != 3).collect();
        if self.mask & LFE != 0 {
            v.push(3);
        }
        v
    }
}

/// `CAkSpeakerPan::GetSpeakerVolumes2DPan` for direct speaker assignment (`0x1415e1730`,
/// panner type 0) and the standard downmix `0x1415dbd30` [V per ADVICE_A3; tables other than
/// mono/stereo output are approximated by 1:1 routing of matching speakers, A].
/// `center` is Center % / 100 (only used for mono input).
pub fn direct_matrix(input: Layout, output: Layout, center: f32) -> (Vec<Vec<f32>>, Vec<&'static str>) {
    let mut notes = vec![];
    let ni = input.channels;
    let no = output.channels;
    let mut m = vec![vec![0.0f32; no]; ni];
    let ospk = output.speakers();
    if input.kind == 2 || output.kind == 2 {
        notes.push("ambisonics");
        return (m, notes);
    }
    let ispk = input.speakers();
    if ispk.is_empty() || ospk.is_empty() {
        // anonymous: identity diagonal (ADVICE_A3 §1.1)
        notes.push("anonymous_layout");
        for (c, row) in m.iter_mut().enumerate().take(no.min(ni)) {
            row[c] = 1.0;
        }
        return (m, notes);
    }
    let opos = |bit: u32| ospk.iter().position(|&b| b == bit);
    if ni == 1 && input.mask == FC {
        // mono source
        if no == 1 {
            m[0][0] = 1.0;
        } else if output.mask & (FL | FR | FC) == (FL | FR | FC) {
            let c = center.clamp(0.0, 1.0);
            let lr = ((1.0 - c) * 0.5).sqrt();
            m[0][opos(0).unwrap()] = lr;
            m[0][opos(1).unwrap()] = lr;
            m[0][opos(2).unwrap()] = c.sqrt();
            if c != 0.0 {
                notes.push("center_pct");
            }
        } else if output.mask & (FL | FR) == (FL | FR) {
            m[0][opos(0).unwrap()] = H;
            m[0][opos(1).unwrap()] = H;
        } else {
            notes.push("mono_to_unusual_layout");
        }
        return (m, notes);
    }
    for (i, &bit) in ispk.iter().enumerate() {
        if bit == 3 {
            // LFE -> LFE only if the output has one, else dropped [V §1.1]
            if let Some(o) = opos(3) {
                m[i][o] = 1.0;
            }
            continue;
        }
        if let Some(o) = opos(bit) {
            m[i][o] = 1.0;
            continue;
        }
        if output.mask == (FL | FR) {
            m[i][0] = TABLE_STEREO[bit as usize][0];
            m[i][1] = TABLE_STEREO[bit as usize][1];
        } else if output.mask == FC {
            m[i][0] = TABLE_MONO[bit as usize];
        } else {
            notes.push("downmix_table_missing");
        }
    }
    (m, notes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layouts() {
        let l = Layout::from_config(0x2d63f10c).unwrap(); // 7.1.4 bus in Init.bnk
        assert_eq!(l.channels, 12);
        assert_eq!(l.mask, 0x2d63f);
        let s = l.speakers();
        assert_eq!(s.len(), 12);
        assert_eq!(*s.last().unwrap(), 3); // LFE last
    }

    #[test]
    fn mono_stereo_and_714_fold() {
        let (m, _) = direct_matrix(Layout::MONO, Layout::STEREO, 0.0);
        assert_eq!(m, vec![vec![H, H]]);
        let l714 = Layout::from_config(0x2d63f10c).unwrap();
        let (m, _) = direct_matrix(l714, Layout::STEREO, 0.0);
        // FL FR FC BL BR SL SR HFL HFR HBL HBR LFE
        assert_eq!(m[0], vec![1.0, 0.0]);
        assert_eq!(m[2], vec![H, H]);
        assert_eq!(m[9], vec![0.5, 0.0]);
        assert_eq!(m[11], vec![0.0, 0.0]);
        let (m, _) = direct_matrix(Layout::MONO, l714, 0.0);
        assert_eq!(m[0][0], 0.5f32.sqrt());
        assert_eq!(m[0][2], 0.0);
    }
}
