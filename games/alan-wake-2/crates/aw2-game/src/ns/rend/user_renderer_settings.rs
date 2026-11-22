//! `rend::UserRendererSettings`.

/// `rend::UserRendererSettings` — field meanings unknown except the defaults
/// written by the constructor (1920x1080 twice, 60, 2.5, -1.0). Padding bytes
/// that the constructor does not write (+0x7f, +0x9e..0xa0, +0xbc..) are kept
/// out of the field list on purpose: the original leaves them untouched.
#[repr(C, packed)]
pub struct UserRendererSettings {
    pub unk_00: [u64; 15],
    pub unk_78: u32,
    pub unk_7c: u16,
    pub unk_7e: u8,
    _pad_7f: u8,
    pub unk_80: u64,
    pub unk_88: f32,
    pub unk_8c: f32,
    pub unk_90: f32,
    pub unk_94: f32,
    pub unk_98: [u8; 4],
    pub unk_9c: u16,
    _pad_9e: [u8; 2],
    pub width_a0: u32,
    pub height_a4: u32,
    pub width_a8: u32,
    pub height_ac: u32,
    pub unk_b0: u32,
    pub unk_b4: u32,
    pub rate_b8: u32,
}

const _: () = {
    use std::mem::offset_of;
    assert!(offset_of!(UserRendererSettings, unk_78) == 0x78);
    assert!(offset_of!(UserRendererSettings, unk_80) == 0x80);
    assert!(offset_of!(UserRendererSettings, unk_88) == 0x88);
    assert!(offset_of!(UserRendererSettings, unk_98) == 0x98);
    assert!(offset_of!(UserRendererSettings, unk_9c) == 0x9c);
    assert!(offset_of!(UserRendererSettings, width_a0) == 0xa0);
    assert!(offset_of!(UserRendererSettings, height_ac) == 0xac);
    assert!(offset_of!(UserRendererSettings, rate_b8) == 0xb8);
};

/// `rend::UserRendererSettings::UserRendererSettings()`.
///
/// Original: 0x1401eecc0 (196 bytes, leaf). Evidence: the only code in
/// `rend::UserRendererSettings::ClassTypeInfoImplementation::vfunc_12`
/// (0x142201560, the ClassTypeInfo "construct in place" slot) is a call to it
/// with the destination buffer. Boot: called twice. Every store is reproduced
/// (the original's 8-byte store of 0x438 at +0xac is split into +0xac/+0xb0).
pub unsafe extern "win64" fn user_renderer_settings_ctor(this: *mut UserRendererSettings) -> *mut UserRendererSettings {
    use std::ptr::addr_of_mut as f;
    f!((*this).unk_88).write_unaligned(2.5); // 0x40200000
    f!((*this).unk_00).write_unaligned([0; 15]);
    f!((*this).unk_78).write_unaligned(0);
    f!((*this).unk_7c).write_unaligned(0);
    f!((*this).unk_7e).write_unaligned(0);
    f!((*this).unk_80).write_unaligned(0);
    f!((*this).unk_9c).write_unaligned(0);
    f!((*this).unk_b4).write_unaligned(0);
    f!((*this).unk_8c).write_unaligned(2.5);
    f!((*this).unk_90).write_unaligned(-1.0); // 0xbf800000
    f!((*this).unk_94).write_unaligned(-1.0);
    f!((*this).unk_98).write_unaligned(0x0101_0100u32.to_le_bytes());
    f!((*this).width_a0).write_unaligned(1920);
    f!((*this).height_a4).write_unaligned(1080);
    f!((*this).width_a8).write_unaligned(1920);
    f!((*this).height_ac).write_unaligned(1080);
    f!((*this).unk_b0).write_unaligned(0);
    f!((*this).rate_b8).write_unaligned(60);
    this
}

pub unsafe fn diff_user_renderer_settings_ctor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut UserRendererSettings) -> *mut UserRendererSettings;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = crate::difftest::Rng::new(0x5e77);
    const SZ: usize = 0xd0; // past the last store (+0xbc) to catch overruns
    for case in 0..64 {
        let init: Vec<u64> = (0..SZ / 8).map(|_| r.next_u64()).collect();
        let (mut a, mut b) = (init.clone(), init.clone());
        // Every other case 4-aligned only (embedded objects).
        let off = 4 * (case % 2);
        let ra = o((a.as_mut_ptr() as *mut u8).add(off) as *mut UserRendererSettings) as usize - a.as_ptr() as usize;
        let rb = n((b.as_mut_ptr() as *mut u8).add(off) as *mut UserRendererSettings) as usize - b.as_ptr() as usize;
        crate::diff_eq!(ra, rb, "return offset, case {case}");
        crate::diff_eq!(a, b, "memory, case {case}");
    }
    Ok(64)
}
