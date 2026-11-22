//! msvcp140_codecvt_ids.dll: only a data export, the `locale::id` object of
//! `std::codecvt<char32_t, char8_t, mbstate_t>`. A zeroed `locale::id`
//! (`size_t _Id`) is lazily numbered by msvcp140 on first use.

pub static ORDINALS: &[(u16, &str)] = &[];

#[repr(C, align(16))]
pub struct LocaleId(std::cell::UnsafeCell<[u64; 2]>);
unsafe impl Sync for LocaleId {}

pub static CODECVT_U32_U8_ID: LocaleId = LocaleId(std::cell::UnsafeCell::new([0; 2]));

pub static EXTRA_EXPORTS: &[crate::Export] = &[crate::Export {
    name: "?id@?$codecvt@_UDU_Mbstatet@@@std@@2V0locale@2@A",
    ptr: crate::Ptr(&CODECVT_U32_U8_ID as *const LocaleId as *const ()),
}];
