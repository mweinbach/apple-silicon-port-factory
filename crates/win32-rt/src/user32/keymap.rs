//! Key tables: macOS virtual key codes (`kVK_*`, positional) → Windows
//! Set-1 scan codes and US-layout virtual keys; `MapVirtualKey`; a US-layout
//! character table for `ToUnicode` and synthetic `TranslateMessage`.
//!
//! VKs are positional (US layout): the key labelled A on an AZERTY keyboard
//! reports VK 'Q', as the game's scan-code based bindings expect. Text
//! input (WM_CHAR) uses the characters AppKit produced, so it follows the
//! active layout.

pub const VK_LBUTTON: u8 = 0x01;
pub const VK_RBUTTON: u8 = 0x02;
pub const VK_MBUTTON: u8 = 0x04;
pub const VK_XBUTTON1: u8 = 0x05;
pub const VK_XBUTTON2: u8 = 0x06;
pub const VK_BACK: u8 = 0x08;
pub const VK_TAB: u8 = 0x09;
pub const VK_RETURN: u8 = 0x0D;
pub const VK_SHIFT: u8 = 0x10;
pub const VK_CONTROL: u8 = 0x11;
pub const VK_MENU: u8 = 0x12;
pub const VK_CAPITAL: u8 = 0x14;
pub const VK_ESCAPE: u8 = 0x1B;
pub const VK_F10: u8 = 0x79;
pub const VK_NUMLOCK: u8 = 0x90;
pub const VK_LSHIFT: u8 = 0xA0;
pub const VK_RSHIFT: u8 = 0xA1;
pub const VK_LCONTROL: u8 = 0xA2;
pub const VK_RCONTROL: u8 = 0xA3;
pub const VK_LMENU: u8 = 0xA4;
pub const VK_RMENU: u8 = 0xA5;

/// One physical key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Key {
    /// macOS `kVK_*` code.
    pub mac: u16,
    /// Windows Set-1 scan code (without the E0 prefix).
    pub scan: u8,
    /// E0-prefixed ("extended") key.
    pub ext: bool,
    /// Virtual key as reported in WM_KEYDOWN (generic VK_SHIFT/CONTROL/MENU for modifiers).
    pub vk: u8,
    /// Left/right specific VK for modifiers (equal to `vk` otherwise).
    pub vk_lr: u8,
}

const fn k(mac: u16, scan: u8, ext: bool, vk: u8) -> Key {
    Key { mac, scan, ext, vk, vk_lr: vk }
}
const fn m(mac: u16, scan: u8, ext: bool, vk: u8, vk_lr: u8) -> Key {
    Key { mac, scan, ext, vk, vk_lr }
}

pub static KEYS: &[Key] = &[
    k(0x00, 0x1E, false, b'A'),
    k(0x01, 0x1F, false, b'S'),
    k(0x02, 0x20, false, b'D'),
    k(0x03, 0x21, false, b'F'),
    k(0x04, 0x23, false, b'H'),
    k(0x05, 0x22, false, b'G'),
    k(0x06, 0x2C, false, b'Z'),
    k(0x07, 0x2D, false, b'X'),
    k(0x08, 0x2E, false, b'C'),
    k(0x09, 0x2F, false, b'V'),
    k(0x0A, 0x56, false, 0xE2), // ISO section -> VK_OEM_102
    k(0x0B, 0x30, false, b'B'),
    k(0x0C, 0x10, false, b'Q'),
    k(0x0D, 0x11, false, b'W'),
    k(0x0E, 0x12, false, b'E'),
    k(0x0F, 0x13, false, b'R'),
    k(0x10, 0x15, false, b'Y'),
    k(0x11, 0x14, false, b'T'),
    k(0x12, 0x02, false, b'1'),
    k(0x13, 0x03, false, b'2'),
    k(0x14, 0x04, false, b'3'),
    k(0x15, 0x05, false, b'4'),
    k(0x16, 0x07, false, b'6'),
    k(0x17, 0x06, false, b'5'),
    k(0x18, 0x0D, false, 0xBB), // = VK_OEM_PLUS
    k(0x19, 0x0A, false, b'9'),
    k(0x1A, 0x08, false, b'7'),
    k(0x1B, 0x0C, false, 0xBD), // - VK_OEM_MINUS
    k(0x1C, 0x09, false, b'8'),
    k(0x1D, 0x0B, false, b'0'),
    k(0x1E, 0x1B, false, 0xDD), // ] VK_OEM_6
    k(0x1F, 0x18, false, b'O'),
    k(0x20, 0x16, false, b'U'),
    k(0x21, 0x1A, false, 0xDB), // [ VK_OEM_4
    k(0x22, 0x17, false, b'I'),
    k(0x23, 0x19, false, b'P'),
    k(0x24, 0x1C, false, VK_RETURN),
    k(0x25, 0x26, false, b'L'),
    k(0x26, 0x24, false, b'J'),
    k(0x27, 0x28, false, 0xDE), // ' VK_OEM_7
    k(0x28, 0x25, false, b'K'),
    k(0x29, 0x27, false, 0xBA), // ; VK_OEM_1
    k(0x2A, 0x2B, false, 0xDC), // \ VK_OEM_5
    k(0x2B, 0x33, false, 0xBC), // , VK_OEM_COMMA
    k(0x2C, 0x35, false, 0xBF), // / VK_OEM_2
    k(0x2D, 0x31, false, b'N'),
    k(0x2E, 0x32, false, b'M'),
    k(0x2F, 0x34, false, 0xBE), // . VK_OEM_PERIOD
    k(0x30, 0x0F, false, VK_TAB),
    k(0x31, 0x39, false, 0x20), // space
    k(0x32, 0x29, false, 0xC0), // ` VK_OEM_3
    k(0x33, 0x0E, false, VK_BACK),
    k(0x35, 0x01, false, VK_ESCAPE),
    m(0x38, 0x2A, false, VK_SHIFT, VK_LSHIFT),
    k(0x39, 0x3A, false, VK_CAPITAL),
    m(0x3A, 0x38, false, VK_MENU, VK_LMENU),
    m(0x3B, 0x1D, false, VK_CONTROL, VK_LCONTROL),
    m(0x3C, 0x36, false, VK_SHIFT, VK_RSHIFT),
    m(0x3D, 0x38, true, VK_MENU, VK_RMENU),
    m(0x3E, 0x1D, true, VK_CONTROL, VK_RCONTROL),
    k(0x40, 0x68, false, 0x80), // F17
    k(0x41, 0x53, false, 0x6E), // keypad . VK_DECIMAL
    k(0x43, 0x37, false, 0x6A), // keypad * VK_MULTIPLY
    k(0x45, 0x4E, false, 0x6B), // keypad + VK_ADD
    k(0x47, 0x45, true, VK_NUMLOCK), // keypad Clear sits where Num Lock is
    k(0x48, 0x30, true, 0xAF), // volume up
    k(0x49, 0x2E, true, 0xAE), // volume down
    k(0x4A, 0x20, true, 0xAD), // mute
    k(0x4B, 0x35, true, 0x6F), // keypad / VK_DIVIDE
    k(0x4C, 0x1C, true, VK_RETURN), // keypad enter
    k(0x4E, 0x4A, false, 0x6D), // keypad - VK_SUBTRACT
    k(0x4F, 0x69, false, 0x81), // F18
    k(0x50, 0x6A, false, 0x82), // F19
    k(0x51, 0x59, false, 0x92), // keypad = VK_OEM_NEC_EQUAL
    k(0x52, 0x52, false, 0x60),
    k(0x53, 0x4F, false, 0x61),
    k(0x54, 0x50, false, 0x62),
    k(0x55, 0x51, false, 0x63),
    k(0x56, 0x4B, false, 0x64),
    k(0x57, 0x4C, false, 0x65),
    k(0x58, 0x4D, false, 0x66),
    k(0x59, 0x47, false, 0x67),
    k(0x5A, 0x6B, false, 0x83), // F20
    k(0x5B, 0x48, false, 0x68),
    k(0x5C, 0x49, false, 0x69),
    k(0x5E, 0x73, false, 0xC1), // JIS underscore -> VK_ABNT_C1
    k(0x5F, 0x7E, false, 0x6C), // JIS keypad comma -> VK_SEPARATOR
    k(0x60, 0x3F, false, 0x74), // F5
    k(0x61, 0x40, false, 0x75), // F6
    k(0x62, 0x41, false, 0x76), // F7
    k(0x63, 0x3D, false, 0x72), // F3
    k(0x64, 0x42, false, 0x77), // F8
    k(0x65, 0x43, false, 0x78), // F9
    k(0x67, 0x57, false, 0x7A), // F11
    k(0x69, 0x64, false, 0x7C), // F13
    k(0x6A, 0x67, false, 0x7F), // F16
    k(0x6B, 0x65, false, 0x7D), // F14
    k(0x6D, 0x44, false, VK_F10),
    k(0x6E, 0x5D, true, 0x5D), // context menu -> VK_APPS
    k(0x6F, 0x58, false, 0x7B), // F12
    k(0x71, 0x66, false, 0x7E), // F15
    k(0x72, 0x52, true, 0x2D), // Help/Insert -> VK_INSERT
    k(0x73, 0x47, true, 0x24), // Home
    k(0x74, 0x49, true, 0x21), // Page Up
    k(0x75, 0x53, true, 0x2E), // Forward delete -> VK_DELETE
    k(0x76, 0x3E, false, 0x73), // F4
    k(0x77, 0x4F, true, 0x23), // End
    k(0x78, 0x3C, false, 0x71), // F2
    k(0x79, 0x51, true, 0x22), // Page Down
    k(0x7A, 0x3B, false, 0x70), // F1
    k(0x7B, 0x4B, true, 0x25), // Left
    k(0x7C, 0x4D, true, 0x27), // Right
    k(0x7D, 0x50, true, 0x28), // Down
    k(0x7E, 0x48, true, 0x26), // Up
];

pub fn from_mac(code: u16) -> Option<&'static Key> {
    KEYS.iter().find(|k| k.mac == code)
}

pub fn from_vk(vk: u32) -> Option<&'static Key> {
    KEYS.iter().find(|k| k.vk_lr as u32 == vk).or_else(|| KEYS.iter().find(|k| k.vk as u32 == vk))
}

pub fn from_scan(scan: u32, ext: bool) -> Option<&'static Key> {
    KEYS.iter().find(|k| k.scan as u32 == scan && k.ext == ext).or_else(|| KEYS.iter().find(|k| k.scan as u32 == scan))
}

/// True for virtual keys that can legitimately produce character output in
/// `ToUnicode` / `ToUnicodeEx` (excludes modifiers, arrows, function keys,
/// lock keys, and mouse buttons so macOS control codes don't leak through and
/// modifier presses never disturb dead-key state).
pub fn is_character_vk(vk: u32) -> bool {
    matches!(
        vk as u8,
        b'A'..=b'Z'
            | b'0'..=b'9'
            | 0x20
            | VK_RETURN
            | VK_TAB
            | VK_BACK
            | VK_ESCAPE
            | 0x60..=0x6F
            | 0x92
            | 0xBA..=0xC1
            | 0xDB..=0xDF
            | 0xE2
    )
}

/// Resolve a Windows `(vk, scan)` pair to a macOS `kVK_*` keycode.
pub fn vk_to_mac(vk: u32, scan: u32) -> Option<u16> {
    if let Some(k) = from_vk(vk) {
        return Some(k.mac);
    }
    let sc = scan & 0xFF;
    if sc != 0 {
        let ext = (scan & 0xFF00) == 0xE000;
        if let Some(k) = from_scan(sc, ext) {
            return Some(k.mac);
        }
    }
    None
}

/// US-layout character for a VK (None for non-character keys).
pub fn us_char(vk: u32, shift: bool, caps: bool) -> Option<u16> {
    let c = match vk as u8 {
        b'A'..=b'Z' => {
            let upper = shift ^ caps;
            if upper {
                vk as u8
            } else {
                vk as u8 + 32
            }
        }
        b'0'..=b'9' => {
            if shift {
                b")!@#$%^&*("[(vk as u8 - b'0') as usize]
            } else {
                vk as u8
            }
        }
        0x20 => b' ',
        VK_RETURN => b'\r',
        VK_TAB => b'\t',
        VK_BACK => 0x08,
        VK_ESCAPE => 0x1B,
        0xBA => if shift { b':' } else { b';' },
        0xBB => if shift { b'+' } else { b'=' },
        0xBC => if shift { b'<' } else { b',' },
        0xBD => if shift { b'_' } else { b'-' },
        0xBE => if shift { b'>' } else { b'.' },
        0xBF => if shift { b'?' } else { b'/' },
        0xC0 => if shift { b'~' } else { b'`' },
        0xDB => if shift { b'{' } else { b'[' },
        0xDC => if shift { b'|' } else { b'\\' },
        0xDD => if shift { b'}' } else { b']' },
        0xDE => if shift { b'"' } else { b'\'' },
        0x60..=0x69 => b'0' + (vk as u8 - 0x60),
        0x6A => b'*',
        0x6B => b'+',
        0x6D => b'-',
        0x6E => b'.',
        0x6F => b'/',
        _ => return None,
    };
    Some(c as u16)
}

pub const MAPVK_VK_TO_VSC: u32 = 0;
pub const MAPVK_VSC_TO_VK: u32 = 1;
pub const MAPVK_VK_TO_CHAR: u32 = 2;
pub const MAPVK_VSC_TO_VK_EX: u32 = 3;
pub const MAPVK_VK_TO_VSC_EX: u32 = 4;

pub fn map_virtual_key(code: u32, ty: u32) -> u32 {
    match ty {
        MAPVK_VK_TO_VSC | MAPVK_VK_TO_VSC_EX => match code as u8 {
            VK_SHIFT => 0x2A,
            VK_CONTROL => 0x1D,
            VK_MENU => 0x38,
            _ => from_vk(code)
                .map(|k| if ty == MAPVK_VK_TO_VSC_EX && k.ext { 0xE000 | k.scan as u32 } else { k.scan as u32 })
                .unwrap_or(0),
        },
        MAPVK_VSC_TO_VK | MAPVK_VSC_TO_VK_EX => {
            let ext = code & 0xFF00 == 0xE000;
            match from_scan(code & 0xFF, ext) {
                Some(k) => {
                    if ty == MAPVK_VSC_TO_VK_EX {
                        k.vk_lr as u32
                    } else {
                        k.vk as u32
                    }
                }
                None => 0,
            }
        }
        MAPVK_VK_TO_CHAR => match us_char(code, false, false) {
            Some(c) if (b'a' as u16..=b'z' as u16).contains(&c) => (c - 32) as u32,
            Some(c) => c as u32,
            None => 0,
        },
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maps() {
        assert_eq!(map_virtual_key(b'A' as u32, MAPVK_VK_TO_VSC), 0x1E);
        assert_eq!(map_virtual_key(0x1E, MAPVK_VSC_TO_VK), b'A' as u32);
        assert_eq!(map_virtual_key(0xE01D, MAPVK_VSC_TO_VK_EX), VK_RCONTROL as u32);
        assert_eq!(map_virtual_key(0x1D, MAPVK_VSC_TO_VK_EX), VK_LCONTROL as u32);
        assert_eq!(map_virtual_key(b'A' as u32, MAPVK_VK_TO_CHAR), b'A' as u32);
        assert_eq!(map_virtual_key(0x26, MAPVK_VK_TO_VSC_EX), 0xE048);
        assert_eq!(us_char(b'1' as u32, true, false), Some(b'!' as u16));
        let mut seen = std::collections::HashSet::new();
        for k in KEYS {
            assert!(seen.insert(k.mac), "duplicate mac code {:#x}", k.mac);
        }
    }
}
