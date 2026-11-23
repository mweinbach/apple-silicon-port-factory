//! Input processor profiles (`ITfInputProcessorProfiles`, `ITfInputProcessorProfileMgr`),
//! category manager (`ITfCategoryMgr`), display attributes (`ITfDisplayAttributeMgr`,
//! `ITfDisplayAttributeInfo`), and language bar manager (`ITfLangBarMgr`).

use super::abi::*;
use super::thread_mgr::current_tid;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{LazyLock, Mutex};

// ---------------------------------------------------------------------------
// `ITfInputProcessorProfiles` (21 slots) + `ITfInputProcessorProfileMgr` (11 slots)
// + `ITfSource` (5 slots) + `IEnumTfLanguageProfiles` + `IEnumTfInputProcessorProfiles`
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct LanguageProfileRecord {
    clsid: Guid,
    langid: u16,
    catid: Guid,
    guid_profile: Guid,
    desc: String,
    enabled: bool,
    enabled_by_default: bool,
    active: bool,
    hkl: HKL,
    hkl_substitute: HKL,
    profile_type: u32,
}

struct ProfilesState {
    current_langid: u16,
    registered_tips: Vec<Guid>,
    profiles: Vec<LanguageProfileRecord>,
    defaults: HashMap<(u16, Guid), (Guid, Guid)>,
}

impl ProfilesState {
    fn new_default() -> Self {
        let mut defaults = HashMap::new();
        defaults.insert(
            (LANGID_EN_US, GUID_TFCAT_TIP_KEYBOARD),
            (CLSID_EN_US_KEYBOARD_TIP, GUID_EN_US_KEYBOARD_PROFILE),
        );
        Self {
            current_langid: LANGID_EN_US,
            registered_tips: vec![CLSID_EN_US_KEYBOARD_TIP],
            profiles: vec![LanguageProfileRecord {
                clsid: CLSID_EN_US_KEYBOARD_TIP,
                langid: LANGID_EN_US,
                catid: GUID_TFCAT_TIP_KEYBOARD,
                guid_profile: GUID_EN_US_KEYBOARD_PROFILE,
                desc: String::from("English (United States)"),
                enabled: true,
                enabled_by_default: true,
                active: true,
                hkl: HKL_EN_US,
                hkl_substitute: HKL_EN_US,
                profile_type: TF_PROFILETYPE_KEYBOARDLAYOUT,
            }],
            defaults,
        }
    }

    fn language_list(&self) -> Vec<u16> {
        let mut langs: Vec<u16> = Vec::new();
        if !langs.contains(&self.current_langid) {
            langs.push(self.current_langid);
        }
        for p in &self.profiles {
            if !langs.contains(&p.langid) {
                langs.push(p.langid);
            }
        }
        langs
    }
}

static PROFILES_STATE: LazyLock<Mutex<ProfilesState>> =
    LazyLock::new(|| Mutex::new(ProfilesState::new_default()));

// Enumerator for TF_LANGUAGEPROFILE (`IEnumTfLanguageProfiles`: Clone, Next, Reset, Skip)
#[repr(C)]
pub(super) struct IEnumTfLanguageProfilesVtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut EnumLangProfilesObj, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut EnumLangProfilesObj) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut EnumLangProfilesObj) -> u32,
    pub(super) clone:
        unsafe extern "win64" fn(*mut EnumLangProfilesObj, *mut *mut c_void) -> HRESULT,
    pub(super) next:
        unsafe extern "win64" fn(*mut EnumLangProfilesObj, u32, *mut u8, *mut u32) -> HRESULT,
    pub(super) reset: unsafe extern "win64" fn(*mut EnumLangProfilesObj) -> HRESULT,
    pub(super) skip: unsafe extern "win64" fn(*mut EnumLangProfilesObj, u32) -> HRESULT,
}
const _: () =
    assert!(std::mem::size_of::<IEnumTfLanguageProfilesVtbl>() == 7 * std::mem::size_of::<usize>());

pub(super) struct EnumLangProfilesInner {
    state: Mutex<(Vec<TF_LANGUAGEPROFILE>, usize)>,
}

pub(super) type EnumLangProfilesObj = ComObj<EnumLangProfilesInner>;

impl_single_qi!(elp_qi, EnumLangProfilesObj, IID_IENUMTFLANGUAGEPROFILES);

unsafe extern "win64" fn elp_clone(
    this: *mut EnumLangProfilesObj,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    let (items, pos) = {
        let st = ComObj::inner(this).state.lock().unwrap();
        (st.0.clone(), st.1)
    };
    std::ptr::write_unaligned(pp_enum, new_enum_lang_profiles(items, pos) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn elp_next(
    this: *mut EnumLangProfilesObj,
    ul_count: u32,
    p_profile: *mut u8,
    pc_fetch: *mut u32,
) -> HRESULT {
    if !pc_fetch.is_null() {
        std::ptr::write_unaligned(pc_fetch, 0);
    }
    if ul_count > 1 && pc_fetch.is_null() {
        return E_INVALIDARG;
    }
    if ul_count > 0 && p_profile.is_null() {
        return E_INVALIDARG;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let avail = st.0.len().saturating_sub(st.1);
    let n = (ul_count as usize).min(avail);
    let stride = std::mem::size_of::<TF_LANGUAGEPROFILE>();
    for i in 0..n {
        std::ptr::write_unaligned(
            p_profile.add(i * stride) as *mut TF_LANGUAGEPROFILE,
            st.0[st.1 + i],
        );
    }
    st.1 += n;
    if !pc_fetch.is_null() {
        std::ptr::write_unaligned(pc_fetch, n as u32);
    }
    if n == ul_count as usize {
        S_OK
    } else {
        S_FALSE
    }
}

unsafe extern "win64" fn elp_reset(this: *mut EnumLangProfilesObj) -> HRESULT {
    ComObj::inner(this).state.lock().unwrap().1 = 0;
    S_OK
}

unsafe extern "win64" fn elp_skip(this: *mut EnumLangProfilesObj, ul_count: u32) -> HRESULT {
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let avail = st.0.len().saturating_sub(st.1);
    let step = (ul_count as usize).min(avail);
    st.1 += step;
    if step == ul_count as usize {
        S_OK
    } else {
        S_FALSE
    }
}

static IENUM_TF_LANGUAGE_PROFILES_VTBL: IEnumTfLanguageProfilesVtbl = IEnumTfLanguageProfilesVtbl {
    query_interface: elp_qi,
    add_ref: com_add_ref::<EnumLangProfilesInner>,
    release: com_release::<EnumLangProfilesInner>,
    clone: elp_clone,
    next: elp_next,
    reset: elp_reset,
    skip: elp_skip,
};

fn new_enum_lang_profiles(items: Vec<TF_LANGUAGEPROFILE>, pos: usize) -> *mut EnumLangProfilesObj {
    register_vtable_ptrs(&[
        elp_qi as *const () as usize,
        com_add_ref::<EnumLangProfilesInner> as *const () as usize,
        com_release::<EnumLangProfilesInner> as *const () as usize,
        elp_clone as *const () as usize,
        elp_next as *const () as usize,
        elp_reset as *const () as usize,
        elp_skip as *const () as usize,
    ]);
    ComObj::new(
        &IENUM_TF_LANGUAGE_PROFILES_VTBL as *const IEnumTfLanguageProfilesVtbl as *const usize,
        EnumLangProfilesInner { state: Mutex::new((items, pos)) },
    )
}

// Enumerator for TF_INPUTPROCESSORPROFILE (`IEnumTfInputProcessorProfiles`)
#[repr(C)]
struct IEnumTfInputProcessorProfilesVtbl {
    query_interface:
        unsafe extern "win64" fn(*mut EnumIppObj, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut EnumIppObj) -> u32,
    release: unsafe extern "win64" fn(*mut EnumIppObj) -> u32,
    clone: unsafe extern "win64" fn(*mut EnumIppObj, *mut *mut c_void) -> HRESULT,
    next: unsafe extern "win64" fn(*mut EnumIppObj, u32, *mut u8, *mut u32) -> HRESULT,
    reset: unsafe extern "win64" fn(*mut EnumIppObj) -> HRESULT,
    skip: unsafe extern "win64" fn(*mut EnumIppObj, u32) -> HRESULT,
}
const _: () = assert!(
    std::mem::size_of::<IEnumTfInputProcessorProfilesVtbl>() == 7 * std::mem::size_of::<usize>()
);

struct EnumIppInner {
    state: Mutex<(Vec<TF_INPUTPROCESSORPROFILE>, usize)>,
}

type EnumIppObj = ComObj<EnumIppInner>;

impl_single_qi!(eipp_qi, EnumIppObj, IID_IENUMTFINPUTPROCESSORPROFILES);

unsafe extern "win64" fn eipp_clone(
    this: *mut EnumIppObj,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    let (items, pos) = {
        let st = ComObj::inner(this).state.lock().unwrap();
        (st.0.clone(), st.1)
    };
    std::ptr::write_unaligned(pp_enum, new_enum_ipp(items, pos) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn eipp_next(
    this: *mut EnumIppObj,
    ul_count: u32,
    p_profile: *mut u8,
    pc_fetch: *mut u32,
) -> HRESULT {
    if !pc_fetch.is_null() {
        std::ptr::write_unaligned(pc_fetch, 0);
    }
    if ul_count > 1 && pc_fetch.is_null() {
        return E_INVALIDARG;
    }
    if ul_count > 0 && p_profile.is_null() {
        return E_INVALIDARG;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let avail = st.0.len().saturating_sub(st.1);
    let n = (ul_count as usize).min(avail);
    let stride = std::mem::size_of::<TF_INPUTPROCESSORPROFILE>();
    for i in 0..n {
        std::ptr::write_unaligned(
            p_profile.add(i * stride) as *mut TF_INPUTPROCESSORPROFILE,
            st.0[st.1 + i],
        );
    }
    st.1 += n;
    if !pc_fetch.is_null() {
        std::ptr::write_unaligned(pc_fetch, n as u32);
    }
    if n == ul_count as usize {
        S_OK
    } else {
        S_FALSE
    }
}

unsafe extern "win64" fn eipp_reset(this: *mut EnumIppObj) -> HRESULT {
    ComObj::inner(this).state.lock().unwrap().1 = 0;
    S_OK
}

unsafe extern "win64" fn eipp_skip(this: *mut EnumIppObj, ul_count: u32) -> HRESULT {
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let avail = st.0.len().saturating_sub(st.1);
    let step = (ul_count as usize).min(avail);
    st.1 += step;
    if step == ul_count as usize {
        S_OK
    } else {
        S_FALSE
    }
}

static IENUM_TF_IPP_VTBL: IEnumTfInputProcessorProfilesVtbl = IEnumTfInputProcessorProfilesVtbl {
    query_interface: eipp_qi,
    add_ref: com_add_ref::<EnumIppInner>,
    release: com_release::<EnumIppInner>,
    clone: eipp_clone,
    next: eipp_next,
    reset: eipp_reset,
    skip: eipp_skip,
};

fn new_enum_ipp(items: Vec<TF_INPUTPROCESSORPROFILE>, pos: usize) -> *mut EnumIppObj {
    register_vtable_ptrs(&[
        eipp_qi as *const () as usize,
        com_add_ref::<EnumIppInner> as *const () as usize,
        com_release::<EnumIppInner> as *const () as usize,
        eipp_clone as *const () as usize,
        eipp_next as *const () as usize,
        eipp_reset as *const () as usize,
        eipp_skip as *const () as usize,
    ]);
    ComObj::new(
        &IENUM_TF_IPP_VTBL as *const IEnumTfInputProcessorProfilesVtbl as *const usize,
        EnumIppInner { state: Mutex::new((items, pos)) },
    )
}

#[repr(C)]
pub(super) struct ITfInputProcessorProfilesVtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) register: unsafe extern "win64" fn(*mut c_void, *const u8) -> HRESULT,
    pub(super) unregister: unsafe extern "win64" fn(*mut c_void, *const u8) -> HRESULT,
    pub(super) add_language_profile: unsafe extern "win64" fn(
        *mut c_void,
        *const u8,
        u16,
        *const u8,
        *const u16,
        u32,
        *const u16,
        u32,
        u32,
    ) -> HRESULT,
    pub(super) remove_language_profile:
        unsafe extern "win64" fn(*mut c_void, *const u8, u16, *const u8) -> HRESULT,
    pub(super) enum_input_processor_info:
        unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    pub(super) get_default_language_profile:
        unsafe extern "win64" fn(*mut c_void, u16, *const u8, *mut u8, *mut u8) -> HRESULT,
    pub(super) set_default_language_profile:
        unsafe extern "win64" fn(*mut c_void, u16, *const u8, *const u8) -> HRESULT,
    pub(super) activate_language_profile:
        unsafe extern "win64" fn(*mut c_void, *const u8, u16, *const u8) -> HRESULT,
    pub(super) get_active_language_profile:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut u16, *mut u8) -> HRESULT,
    pub(super) get_language_profile_description:
        unsafe extern "win64" fn(*mut c_void, *const u8, u16, *const u8, *mut *mut u16) -> HRESULT,
    pub(super) get_current_language: unsafe extern "win64" fn(*mut c_void, *mut u16) -> HRESULT,
    pub(super) change_current_language: unsafe extern "win64" fn(*mut c_void, u16) -> HRESULT,
    pub(super) get_language_list:
        unsafe extern "win64" fn(*mut c_void, *mut *mut u16, *mut u32) -> HRESULT,
    pub(super) enum_language_profiles:
        unsafe extern "win64" fn(*mut c_void, u16, *mut *mut c_void) -> HRESULT,
    pub(super) enable_language_profile:
        unsafe extern "win64" fn(*mut c_void, *const u8, u16, *const u8, BOOL) -> HRESULT,
    pub(super) is_enabled_language_profile:
        unsafe extern "win64" fn(*mut c_void, *const u8, u16, *const u8, *mut BOOL) -> HRESULT,
    pub(super) enable_language_profile_by_default:
        unsafe extern "win64" fn(*mut c_void, *const u8, u16, *const u8, BOOL) -> HRESULT,
    pub(super) substitute_keyboard_layout:
        unsafe extern "win64" fn(*mut c_void, *const u8, u16, *const u8, HKL) -> HRESULT,
}
const _: () = assert!(
    std::mem::size_of::<ITfInputProcessorProfilesVtbl>() == 21 * std::mem::size_of::<usize>()
);

#[repr(C)]
pub(super) struct ITfInputProcessorProfileMgrVtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) activate_profile: unsafe extern "win64" fn(
        *mut c_void,
        u32,
        u16,
        *const u8,
        *const u8,
        HKL,
        u32,
    ) -> HRESULT,
    pub(super) deactivate_profile: unsafe extern "win64" fn(
        *mut c_void,
        u32,
        u16,
        *const u8,
        *const u8,
        HKL,
        u32,
    ) -> HRESULT,
    pub(super) get_profile: unsafe extern "win64" fn(
        *mut c_void,
        u32,
        u16,
        *const u8,
        *const u8,
        HKL,
        *mut u8,
    ) -> HRESULT,
    pub(super) enum_profiles:
        unsafe extern "win64" fn(*mut c_void, u16, *mut *mut c_void) -> HRESULT,
    pub(super) release_input_processor:
        unsafe extern "win64" fn(*mut c_void, *const u8, u32) -> HRESULT,
    pub(super) register_profile: unsafe extern "win64" fn(
        *mut c_void,
        *const u8,
        u16,
        *const u8,
        *const u16,
        u32,
        *const u16,
        u32,
        u32,
        HKL,
        u32,
        BOOL,
        u32,
    ) -> HRESULT,
    pub(super) unregister_profile:
        unsafe extern "win64" fn(*mut c_void, *const u8, u16, *const u8, u32) -> HRESULT,
    pub(super) get_active_profile:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut u8) -> HRESULT,
}
const _: () = assert!(
    std::mem::size_of::<ITfInputProcessorProfileMgrVtbl>() == 11 * std::mem::size_of::<usize>()
);

#[repr(C)]
pub(super) struct InputProcessorProfilesObj {
    vtbl_profiles: *const ITfInputProcessorProfilesVtbl,
    vtbl_profile_mgr: *const ITfInputProcessorProfileMgrVtbl,
    vtbl_source: *const ITfSourceVtbl,
    refs: AtomicU32,
    sinks: Mutex<SinkTable>,
}

pub(super) fn ipp_supports_iid(iid: &Guid) -> bool {
    *iid == IID_IUNKNOWN
        || *iid == IID_ITFINPUTPROCESSORPROFILES
        || *iid == IID_ITFINPUTPROCESSORPROFILEMGR
        || *iid == IID_ITFSOURCE
}

pub(super) unsafe fn ipp_qi_common(
    base: *mut InputProcessorProfilesObj,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    let b = base as *mut u8;
    qi_multi_iface(
        &(*base).refs,
        riid,
        ppv,
        base as *mut c_void,
        &[
            (IID_ITFINPUTPROCESSORPROFILES, base as *mut c_void),
            (
                IID_ITFINPUTPROCESSORPROFILEMGR,
                b.add(std::mem::offset_of!(InputProcessorProfilesObj, vtbl_profile_mgr))
                    as *mut c_void,
            ),
            (
                IID_ITFSOURCE,
                b.add(std::mem::offset_of!(InputProcessorProfilesObj, vtbl_source)) as *mut c_void,
            ),
        ],
    )
}

unsafe fn ipp_add_ref_common(base: *mut InputProcessorProfilesObj) -> u32 {
    (*base).refs.fetch_add(1, Ordering::AcqRel) + 1
}

pub(super) unsafe fn ipp_release_common(base: *mut InputProcessorProfilesObj) -> u32 {
    let obj = &*base;
    let rem = obj.refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if rem == 0 {
        let sinks = obj.sinks.lock().unwrap().drain_all();
        drop(Box::from_raw(base));
        release_all_ptrs(sinks);
    }
    rem
}

impl_primary_iunknown!(
    ipp_qi,
    ipp_add_ref,
    ipp_release,
    InputProcessorProfilesObj,
    ipp_qi_common,
    ipp_add_ref_common,
    ipp_release_common
);
impl_sub_iunknown!(
    ipp_from_mgr,
    ippm_qi,
    ippm_add_ref,
    ippm_release,
    InputProcessorProfilesObj,
    vtbl_profile_mgr,
    ipp_qi_common,
    ipp_add_ref_common,
    ipp_release_common
);
impl_sub_iunknown!(
    ipp_from_source,
    ipp_src_qi,
    ipp_src_add_ref,
    ipp_src_release,
    InputProcessorProfilesObj,
    vtbl_source,
    ipp_qi_common,
    ipp_add_ref_common,
    ipp_release_common
);

fn to_tf_ipp(r: &LanguageProfileRecord) -> TF_INPUTPROCESSORPROFILE {
    let mut flags = 0u32;
    if r.active {
        flags |= TF_IPP_FLAG_ACTIVE;
    }
    if r.enabled {
        flags |= TF_IPP_FLAG_ENABLED;
    }
    TF_INPUTPROCESSORPROFILE {
        dw_profile_type: r.profile_type,
        langid: r.langid,
        clsid: r.clsid,
        guid_profile: r.guid_profile,
        catid: r.catid,
        hkl_substitute: r.hkl_substitute,
        dw_caps: 0,
        hkl: r.hkl,
        dw_flags: flags,
    }
}

unsafe extern "win64" fn ipp_register(_this: *mut c_void, rclsid: *const u8) -> HRESULT {
    let Some(clsid) = read_guid_from_bytes(rclsid) else {
        return E_INVALIDARG;
    };
    let mut st = PROFILES_STATE.lock().unwrap();
    if !st.registered_tips.contains(&clsid) {
        st.registered_tips.push(clsid);
    }
    S_OK
}

unsafe extern "win64" fn ipp_unregister(_this: *mut c_void, rclsid: *const u8) -> HRESULT {
    let Some(clsid) = read_guid_from_bytes(rclsid) else {
        return E_INVALIDARG;
    };
    let mut st = PROFILES_STATE.lock().unwrap();
    st.registered_tips.retain(|g| *g != clsid);
    st.profiles.retain(|p| p.clsid != clsid);
    S_OK
}

unsafe extern "win64" fn ipp_add_lang_profile(
    _this: *mut c_void,
    rclsid: *const u8,
    langid: u16,
    guid_profile: *const u8,
    pch_desc: *const u16,
    cch_desc: u32,
    _pch_icon_file: *const u16,
    _cch_file: u32,
    _u_icon_index: u32,
) -> HRESULT {
    let (Some(clsid), Some(gp)) = (read_guid_from_bytes(rclsid), read_guid_from_bytes(guid_profile))
    else {
        return E_INVALIDARG;
    };
    let desc = read_wide_len(pch_desc, cch_desc as usize);
    let mut st = PROFILES_STATE.lock().unwrap();
    if !st.registered_tips.contains(&clsid) {
        st.registered_tips.push(clsid);
    }
    if let Some(existing) = st
        .profiles
        .iter_mut()
        .find(|p| p.clsid == clsid && p.langid == langid && p.guid_profile == gp)
    {
        existing.desc = desc;
    } else {
        st.profiles.push(LanguageProfileRecord {
            clsid,
            langid,
            catid: GUID_TFCAT_TIP_KEYBOARD,
            guid_profile: gp,
            desc,
            enabled: true,
            enabled_by_default: true,
            active: false,
            hkl: ((langid as usize) << 16) | (langid as usize),
            hkl_substitute: 0,
            profile_type: TF_PROFILETYPE_INPUTPROCESSOR,
        });
    }
    S_OK
}

unsafe extern "win64" fn ipp_remove_lang_profile(
    _this: *mut c_void,
    rclsid: *const u8,
    langid: u16,
    guid_profile: *const u8,
) -> HRESULT {
    let (Some(clsid), Some(gp)) = (read_guid_from_bytes(rclsid), read_guid_from_bytes(guid_profile))
    else {
        return E_INVALIDARG;
    };
    let mut st = PROFILES_STATE.lock().unwrap();
    st.profiles
        .retain(|p| !(p.clsid == clsid && p.langid == langid && p.guid_profile == gp));
    S_OK
}

unsafe extern "win64" fn ipp_enum_info(_this: *mut c_void, pp_enum: *mut *mut c_void) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    let tips = PROFILES_STATE.lock().unwrap().registered_tips.clone();
    std::ptr::write_unaligned(pp_enum, new_enum_guid(tips, 0) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn ipp_get_default_lang_profile(
    _this: *mut c_void,
    langid: u16,
    catid: *const u8,
    pclsid: *mut u8,
    pguid_profile: *mut u8,
) -> HRESULT {
    write_guid(pclsid as *mut Guid, GUID_NULL);
    write_guid(pguid_profile as *mut Guid, GUID_NULL);
    let Some(cat) = read_guid_from_bytes(catid) else {
        return E_INVALIDARG;
    };
    if pclsid.is_null() || pguid_profile.is_null() {
        return E_INVALIDARG;
    }
    let st = PROFILES_STATE.lock().unwrap();
    let target_lang = if langid == 0 { st.current_langid } else { langid };
    if let Some(&(clsid, gp)) = st.defaults.get(&(target_lang, cat)) {
        write_guid(pclsid as *mut Guid, clsid);
        write_guid(pguid_profile as *mut Guid, gp);
        S_OK
    } else {
        S_FALSE
    }
}

unsafe extern "win64" fn ipp_set_default_lang_profile(
    _this: *mut c_void,
    langid: u16,
    rclsid: *const u8,
    guid_profile: *const u8,
) -> HRESULT {
    let (Some(clsid), Some(gp)) = (read_guid_from_bytes(rclsid), read_guid_from_bytes(guid_profile))
    else {
        return E_INVALIDARG;
    };
    let mut st = PROFILES_STATE.lock().unwrap();
    st.defaults.insert((langid, GUID_TFCAT_TIP_KEYBOARD), (clsid, gp));
    S_OK
}

unsafe extern "win64" fn ipp_activate_lang_profile(
    _this: *mut c_void,
    rclsid: *const u8,
    langid: u16,
    guid_profile: *const u8,
) -> HRESULT {
    let (Some(clsid), Some(gp)) = (read_guid_from_bytes(rclsid), read_guid_from_bytes(guid_profile))
    else {
        return E_INVALIDARG;
    };
    let mut st = PROFILES_STATE.lock().unwrap();
    let mut found = false;
    for p in &mut st.profiles {
        if p.clsid == clsid && p.langid == langid && p.guid_profile == gp {
            p.active = true;
            found = true;
        } else if p.langid == langid {
            p.active = false;
        }
    }
    if found {
        st.current_langid = langid;
        S_OK
    } else {
        E_INVALIDARG
    }
}

unsafe extern "win64" fn ipp_get_active_lang_profile(
    _this: *mut c_void,
    rclsid: *const u8,
    plangid: *mut u16,
    pguid_profile: *mut u8,
) -> HRESULT {
    if !plangid.is_null() {
        std::ptr::write_unaligned(plangid, 0);
    }
    write_guid(pguid_profile as *mut Guid, GUID_NULL);
    let Some(clsid) = read_guid_from_bytes(rclsid) else {
        return E_INVALIDARG;
    };
    if plangid.is_null() || pguid_profile.is_null() {
        return E_INVALIDARG;
    }
    let st = PROFILES_STATE.lock().unwrap();
    if let Some(p) = st.profiles.iter().find(|p| p.clsid == clsid && p.active) {
        std::ptr::write_unaligned(plangid, p.langid);
        write_guid(pguid_profile as *mut Guid, p.guid_profile);
        S_OK
    } else {
        S_FALSE
    }
}

unsafe extern "win64" fn ipp_get_lang_profile_desc(
    _this: *mut c_void,
    rclsid: *const u8,
    langid: u16,
    guid_profile: *const u8,
    pbstr_profile: *mut *mut u16,
) -> HRESULT {
    write_null_out(pbstr_profile);
    let (Some(clsid), Some(gp)) = (read_guid_from_bytes(rclsid), read_guid_from_bytes(guid_profile))
    else {
        return E_INVALIDARG;
    };
    if pbstr_profile.is_null() {
        return E_INVALIDARG;
    }
    let desc = PROFILES_STATE
        .lock()
        .unwrap()
        .profiles
        .iter()
        .find(|p| p.clsid == clsid && p.langid == langid && p.guid_profile == gp)
        .map(|p| p.desc.clone());
    match desc {
        Some(s) => {
            std::ptr::write_unaligned(pbstr_profile, alloc_bstr_str(&s));
            S_OK
        }
        None => E_FAIL,
    }
}

unsafe extern "win64" fn ipp_get_current_language(
    _this: *mut c_void,
    plangid: *mut u16,
) -> HRESULT {
    if plangid.is_null() {
        return E_INVALIDARG;
    }
    let lang = PROFILES_STATE.lock().unwrap().current_langid;
    std::ptr::write_unaligned(plangid, lang);
    S_OK
}

unsafe extern "win64" fn ipp_change_current_language(_this: *mut c_void, langid: u16) -> HRESULT {
    let mut st = PROFILES_STATE.lock().unwrap();
    st.current_langid = langid;
    S_OK
}

unsafe extern "win64" fn ipp_get_language_list(
    _this: *mut c_void,
    pp_lang_id: *mut *mut u16,
    pul_count: *mut u32,
) -> HRESULT {
    write_null_out(pp_lang_id);
    if !pul_count.is_null() {
        std::ptr::write_unaligned(pul_count, 0);
    }
    if pp_lang_id.is_null() || pul_count.is_null() {
        return E_INVALIDARG;
    }
    let langs = PROFILES_STATE.lock().unwrap().language_list();
    let bytes = langs.len() * std::mem::size_of::<u16>();
    let buf = crate::misc::ole32::CoTaskMemAlloc(bytes.max(2)) as *mut u16;
    if buf.is_null() {
        return E_OUTOFMEMORY;
    }
    for (i, &lang) in langs.iter().enumerate() {
        std::ptr::write_unaligned(buf.add(i), lang);
    }
    std::ptr::write_unaligned(pp_lang_id, buf);
    std::ptr::write_unaligned(pul_count, langs.len() as u32);
    S_OK
}

unsafe extern "win64" fn ipp_enum_lang_profiles(
    _this: *mut c_void,
    langid: u16,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    let st = PROFILES_STATE.lock().unwrap();
    let items: Vec<TF_LANGUAGEPROFILE> = st
        .profiles
        .iter()
        .filter(|p| langid == 0 || p.langid == langid)
        .map(|p| TF_LANGUAGEPROFILE {
            clsid: p.clsid,
            langid: p.langid,
            catid: p.catid,
            f_active: if p.active { TRUE } else { FALSE },
            guid_profile: p.guid_profile,
        })
        .collect();
    std::ptr::write_unaligned(pp_enum, new_enum_lang_profiles(items, 0) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn ipp_enable_lang_profile(
    _this: *mut c_void,
    rclsid: *const u8,
    langid: u16,
    guid_profile: *const u8,
    f_enable: BOOL,
) -> HRESULT {
    let (Some(clsid), Some(gp)) = (read_guid_from_bytes(rclsid), read_guid_from_bytes(guid_profile))
    else {
        return E_INVALIDARG;
    };
    let mut st = PROFILES_STATE.lock().unwrap();
    if let Some(p) = st
        .profiles
        .iter_mut()
        .find(|p| p.clsid == clsid && p.langid == langid && p.guid_profile == gp)
    {
        p.enabled = f_enable != 0;
        S_OK
    } else {
        E_INVALIDARG
    }
}

unsafe extern "win64" fn ipp_is_enabled_lang_profile(
    _this: *mut c_void,
    rclsid: *const u8,
    langid: u16,
    guid_profile: *const u8,
    pf_enable: *mut BOOL,
) -> HRESULT {
    if !pf_enable.is_null() {
        std::ptr::write_unaligned(pf_enable, FALSE);
    }
    let (Some(clsid), Some(gp)) = (read_guid_from_bytes(rclsid), read_guid_from_bytes(guid_profile))
    else {
        return E_INVALIDARG;
    };
    if pf_enable.is_null() {
        return E_INVALIDARG;
    }
    let st = PROFILES_STATE.lock().unwrap();
    let enabled = st
        .profiles
        .iter()
        .find(|p| p.clsid == clsid && p.langid == langid && p.guid_profile == gp)
        .is_some_and(|p| p.enabled);
    std::ptr::write_unaligned(pf_enable, if enabled { TRUE } else { FALSE });
    S_OK
}

unsafe extern "win64" fn ipp_enable_lang_profile_by_default(
    _this: *mut c_void,
    rclsid: *const u8,
    langid: u16,
    guid_profile: *const u8,
    f_enable: BOOL,
) -> HRESULT {
    let (Some(clsid), Some(gp)) = (read_guid_from_bytes(rclsid), read_guid_from_bytes(guid_profile))
    else {
        return E_INVALIDARG;
    };
    let mut st = PROFILES_STATE.lock().unwrap();
    if let Some(p) = st
        .profiles
        .iter_mut()
        .find(|p| p.clsid == clsid && p.langid == langid && p.guid_profile == gp)
    {
        p.enabled_by_default = f_enable != 0;
        S_OK
    } else {
        E_INVALIDARG
    }
}

unsafe extern "win64" fn ipp_substitute_keyboard_layout(
    _this: *mut c_void,
    rclsid: *const u8,
    langid: u16,
    guid_profile: *const u8,
    hkl: HKL,
) -> HRESULT {
    let (Some(clsid), Some(gp)) = (read_guid_from_bytes(rclsid), read_guid_from_bytes(guid_profile))
    else {
        return E_INVALIDARG;
    };
    let mut st = PROFILES_STATE.lock().unwrap();
    if let Some(p) = st
        .profiles
        .iter_mut()
        .find(|p| p.clsid == clsid && p.langid == langid && p.guid_profile == gp)
    {
        p.hkl_substitute = hkl;
        S_OK
    } else {
        E_INVALIDARG
    }
}

// ITfInputProcessorProfileMgr thunks
unsafe extern "win64" fn ippm_activate_profile(
    _this: *mut c_void,
    _dw_profile_type: u32,
    langid: u16,
    clsid: *const u8,
    guid_profile: *const u8,
    _hkl: HKL,
    _dw_flags: u32,
) -> HRESULT {
    let (Some(c), Some(gp)) = (read_guid_from_bytes(clsid), read_guid_from_bytes(guid_profile))
    else {
        return E_INVALIDARG;
    };
    let mut st = PROFILES_STATE.lock().unwrap();
    for p in &mut st.profiles {
        p.active = p.langid == langid && (p.clsid == c && p.guid_profile == gp);
    }
    st.current_langid = langid;
    S_OK
}

unsafe extern "win64" fn ippm_deactivate_profile(
    _this: *mut c_void,
    _dw_profile_type: u32,
    langid: u16,
    clsid: *const u8,
    guid_profile: *const u8,
    _hkl: HKL,
    _dw_flags: u32,
) -> HRESULT {
    let (Some(c), Some(gp)) = (read_guid_from_bytes(clsid), read_guid_from_bytes(guid_profile))
    else {
        return E_INVALIDARG;
    };
    let mut st = PROFILES_STATE.lock().unwrap();
    for p in &mut st.profiles {
        if p.langid == langid && p.clsid == c && p.guid_profile == gp {
            p.active = false;
        }
    }
    S_OK
}

unsafe extern "win64" fn ippm_get_profile(
    _this: *mut c_void,
    dw_profile_type: u32,
    langid: u16,
    clsid: *const u8,
    guid_profile: *const u8,
    hkl: HKL,
    p_profile: *mut u8,
) -> HRESULT {
    if p_profile.is_null() {
        return E_INVALIDARG;
    }
    let c = read_guid_from_bytes(clsid).unwrap_or(GUID_NULL);
    let gp = read_guid_from_bytes(guid_profile).unwrap_or(GUID_NULL);
    let st = PROFILES_STATE.lock().unwrap();
    let found = st.profiles.iter().find(|p| {
        (dw_profile_type == 0 || p.profile_type == dw_profile_type)
            && (langid == 0 || p.langid == langid)
            && (c == GUID_NULL || p.clsid == c)
            && (gp == GUID_NULL || p.guid_profile == gp)
            && (hkl == 0 || p.hkl == hkl)
    });
    match found {
        Some(r) => {
            std::ptr::write_unaligned(p_profile as *mut TF_INPUTPROCESSORPROFILE, to_tf_ipp(r));
            S_OK
        }
        None => E_FAIL,
    }
}

unsafe extern "win64" fn ippm_enum_profiles(
    _this: *mut c_void,
    langid: u16,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    let st = PROFILES_STATE.lock().unwrap();
    let items: Vec<TF_INPUTPROCESSORPROFILE> = st
        .profiles
        .iter()
        .filter(|p| langid == 0 || p.langid == langid)
        .map(to_tf_ipp)
        .collect();
    std::ptr::write_unaligned(pp_enum, new_enum_ipp(items, 0) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn ippm_release_input_processor(
    _this: *mut c_void,
    _rclsid: *const u8,
    _dw_flags: u32,
) -> HRESULT {
    S_OK
}

unsafe extern "win64" fn ippm_register_profile(
    _this: *mut c_void,
    rclsid: *const u8,
    langid: u16,
    guid_profile: *const u8,
    pch_desc: *const u16,
    cch_desc: u32,
    _pch_icon_file: *const u16,
    _cch_file: u32,
    _u_icon_index: u32,
    hkl_substitute: HKL,
    _dw_preferred_layout: u32,
    b_enabled_by_default: BOOL,
    _dw_flags: u32,
) -> HRESULT {
    let (Some(clsid), Some(gp)) = (read_guid_from_bytes(rclsid), read_guid_from_bytes(guid_profile))
    else {
        return E_INVALIDARG;
    };
    let desc = read_wide_len(pch_desc, cch_desc as usize);
    let mut st = PROFILES_STATE.lock().unwrap();
    if !st.registered_tips.contains(&clsid) {
        st.registered_tips.push(clsid);
    }
    st.profiles.push(LanguageProfileRecord {
        clsid,
        langid,
        catid: GUID_TFCAT_TIP_KEYBOARD,
        guid_profile: gp,
        desc,
        enabled: b_enabled_by_default != 0,
        enabled_by_default: b_enabled_by_default != 0,
        active: false,
        hkl: ((langid as usize) << 16) | (langid as usize),
        hkl_substitute,
        profile_type: TF_PROFILETYPE_INPUTPROCESSOR,
    });
    S_OK
}

unsafe extern "win64" fn ippm_unregister_profile(
    _this: *mut c_void,
    rclsid: *const u8,
    langid: u16,
    guid_profile: *const u8,
    _dw_flags: u32,
) -> HRESULT {
    let (Some(clsid), Some(gp)) = (read_guid_from_bytes(rclsid), read_guid_from_bytes(guid_profile))
    else {
        return E_INVALIDARG;
    };
    let mut st = PROFILES_STATE.lock().unwrap();
    st.profiles
        .retain(|p| !(p.clsid == clsid && p.langid == langid && p.guid_profile == gp));
    S_OK
}

unsafe extern "win64" fn ippm_get_active_profile(
    _this: *mut c_void,
    catid: *const u8,
    p_profile: *mut u8,
) -> HRESULT {
    let Some(cat) = read_guid_from_bytes(catid) else {
        return E_INVALIDARG;
    };
    if p_profile.is_null() {
        return E_INVALIDARG;
    }
    let st = PROFILES_STATE.lock().unwrap();
    let found = st
        .profiles
        .iter()
        .find(|p| p.catid == cat && p.active)
        .or_else(|| st.profiles.iter().find(|p| p.catid == cat));
    match found {
        Some(r) => {
            std::ptr::write_unaligned(p_profile as *mut TF_INPUTPROCESSORPROFILE, to_tf_ipp(r));
            S_OK
        }
        None => S_FALSE,
    }
}

// ITfSource thunks on InputProcessorProfilesObj
unsafe extern "win64" fn ipp_src_advise(
    this: *mut c_void,
    riid: *const u8,
    punk: *mut c_void,
    pdw_cookie: *mut u32,
) -> HRESULT {
    let obj = &*ipp_from_source(this);
    sink_advise(&obj.sinks, riid, punk, pdw_cookie)
}

unsafe extern "win64" fn ipp_src_unadvise(this: *mut c_void, cookie: u32) -> HRESULT {
    let obj = &*ipp_from_source(this);
    sink_unadvise(&obj.sinks, cookie)
}

static ITF_IPP_VTBL: ITfInputProcessorProfilesVtbl = ITfInputProcessorProfilesVtbl {
    query_interface: ipp_qi,
    add_ref: ipp_add_ref,
    release: ipp_release,
    register: ipp_register,
    unregister: ipp_unregister,
    add_language_profile: ipp_add_lang_profile,
    remove_language_profile: ipp_remove_lang_profile,
    enum_input_processor_info: ipp_enum_info,
    get_default_language_profile: ipp_get_default_lang_profile,
    set_default_language_profile: ipp_set_default_lang_profile,
    activate_language_profile: ipp_activate_lang_profile,
    get_active_language_profile: ipp_get_active_lang_profile,
    get_language_profile_description: ipp_get_lang_profile_desc,
    get_current_language: ipp_get_current_language,
    change_current_language: ipp_change_current_language,
    get_language_list: ipp_get_language_list,
    enum_language_profiles: ipp_enum_lang_profiles,
    enable_language_profile: ipp_enable_lang_profile,
    is_enabled_language_profile: ipp_is_enabled_lang_profile,
    enable_language_profile_by_default: ipp_enable_lang_profile_by_default,
    substitute_keyboard_layout: ipp_substitute_keyboard_layout,
};

static ITF_IPPM_VTBL: ITfInputProcessorProfileMgrVtbl = ITfInputProcessorProfileMgrVtbl {
    query_interface: ippm_qi,
    add_ref: ippm_add_ref,
    release: ippm_release,
    activate_profile: ippm_activate_profile,
    deactivate_profile: ippm_deactivate_profile,
    get_profile: ippm_get_profile,
    enum_profiles: ippm_enum_profiles,
    release_input_processor: ippm_release_input_processor,
    register_profile: ippm_register_profile,
    unregister_profile: ippm_unregister_profile,
    get_active_profile: ippm_get_active_profile,
};

static ITF_IPP_SRC_VTBL: ITfSourceVtbl = ITfSourceVtbl {
    query_interface: ipp_src_qi,
    add_ref: ipp_src_add_ref,
    release: ipp_src_release,
    advise_sink: ipp_src_advise,
    unadvise_sink: ipp_src_unadvise,
};

pub(super) fn new_input_processor_profiles_obj() -> *mut InputProcessorProfilesObj {
    register_vtable_ptrs(&[
        ipp_qi as *const () as usize,
        ipp_add_ref as *const () as usize,
        ipp_release as *const () as usize,
        ipp_register as *const () as usize,
        ipp_unregister as *const () as usize,
        ipp_add_lang_profile as *const () as usize,
        ipp_remove_lang_profile as *const () as usize,
        ipp_enum_info as *const () as usize,
        ipp_get_default_lang_profile as *const () as usize,
        ipp_set_default_lang_profile as *const () as usize,
        ipp_activate_lang_profile as *const () as usize,
        ipp_get_active_lang_profile as *const () as usize,
        ipp_get_lang_profile_desc as *const () as usize,
        ipp_get_current_language as *const () as usize,
        ipp_change_current_language as *const () as usize,
        ipp_get_language_list as *const () as usize,
        ipp_enum_lang_profiles as *const () as usize,
        ipp_enable_lang_profile as *const () as usize,
        ipp_is_enabled_lang_profile as *const () as usize,
        ipp_enable_lang_profile_by_default as *const () as usize,
        ipp_substitute_keyboard_layout as *const () as usize,
        ippm_qi as *const () as usize,
        ippm_add_ref as *const () as usize,
        ippm_release as *const () as usize,
        ippm_activate_profile as *const () as usize,
        ippm_deactivate_profile as *const () as usize,
        ippm_get_profile as *const () as usize,
        ippm_enum_profiles as *const () as usize,
        ippm_release_input_processor as *const () as usize,
        ippm_register_profile as *const () as usize,
        ippm_unregister_profile as *const () as usize,
        ippm_get_active_profile as *const () as usize,
        ipp_src_qi as *const () as usize,
        ipp_src_add_ref as *const () as usize,
        ipp_src_release as *const () as usize,
        ipp_src_advise as *const () as usize,
        ipp_src_unadvise as *const () as usize,
    ]);
    Box::into_raw(Box::new(InputProcessorProfilesObj {
        vtbl_profiles: &ITF_IPP_VTBL,
        vtbl_profile_mgr: &ITF_IPPM_VTBL,
        vtbl_source: &ITF_IPP_SRC_VTBL,
        refs: AtomicU32::new(1),
        sinks: Mutex::new(SinkTable::default()),
    }))
}

// ---------------------------------------------------------------------------
// `ITfCategoryMgr` (17 slots)
// ---------------------------------------------------------------------------

struct CategoryState {
    next_atom: TfGuidAtom,
    guid_to_atom: HashMap<Guid, TfGuidAtom>,
    atom_to_guid: HashMap<TfGuidAtom, Guid>,
    /// `(clsid, catid, rguid)` registrations.
    categories: Vec<(Guid, Guid, Guid)>,
    descriptions: HashMap<Guid, String>,
    dwords: HashMap<Guid, u32>,
}

impl Default for CategoryState {
    fn default() -> Self {
        Self {
            next_atom: 1,
            guid_to_atom: HashMap::new(),
            atom_to_guid: HashMap::new(),
            categories: vec![(
                CLSID_EN_US_KEYBOARD_TIP,
                GUID_TFCAT_TIP_KEYBOARD,
                CLSID_EN_US_KEYBOARD_TIP,
            )],
            descriptions: HashMap::new(),
            dwords: HashMap::new(),
        }
    }
}

static CATEGORY_STATE: LazyLock<Mutex<CategoryState>> =
    LazyLock::new(|| Mutex::new(CategoryState::default()));

#[repr(C)]
pub(super) struct ITfCategoryMgrVtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut CategoryMgrObj) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut CategoryMgrObj) -> u32,
    pub(super) register_category:
        unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *const u8, *const u8) -> HRESULT,
    pub(super) unregister_category:
        unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *const u8, *const u8) -> HRESULT,
    pub(super) enum_categories_in_item:
        unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) enum_items_in_category:
        unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) find_closest_category: unsafe extern "win64" fn(
        *mut CategoryMgrObj,
        *const u8,
        *mut u8,
        *const *const u8,
        u32,
    ) -> HRESULT,
    pub(super) register_guid_description: unsafe extern "win64" fn(
        *mut CategoryMgrObj,
        *const u8,
        *const u8,
        *const u16,
        u32,
    ) -> HRESULT,
    pub(super) unregister_guid_description:
        unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *const u8) -> HRESULT,
    pub(super) get_guid_description:
        unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *mut *mut u16) -> HRESULT,
    pub(super) register_guid_dword:
        unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *const u8, u32) -> HRESULT,
    pub(super) unregister_guid_dword:
        unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *const u8) -> HRESULT,
    pub(super) get_guid_dword:
        unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *mut u32) -> HRESULT,
    pub(super) register_guid:
        unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *mut TfGuidAtom) -> HRESULT,
    pub(super) get_guid: unsafe extern "win64" fn(*mut CategoryMgrObj, TfGuidAtom, *mut u8) -> HRESULT,
    pub(super) is_equal_tf_guid_atom:
        unsafe extern "win64" fn(*mut CategoryMgrObj, TfGuidAtom, *const u8, *mut BOOL) -> HRESULT,
}
const _: () =
    assert!(std::mem::size_of::<ITfCategoryMgrVtbl>() == 17 * std::mem::size_of::<usize>());

pub(super) struct CategoryMgrInner;

pub(super) type CategoryMgrObj = ComObj<CategoryMgrInner>;

impl_single_qi!(cat_qi, CategoryMgrObj, IID_ITFCATEGORYMGR);

unsafe extern "win64" fn cat_register_category(
    _this: *mut CategoryMgrObj,
    rclsid: *const u8,
    rcatid: *const u8,
    rguid: *const u8,
) -> HRESULT {
    let (Some(c), Some(cat), Some(g)) = (
        read_guid_from_bytes(rclsid),
        read_guid_from_bytes(rcatid),
        read_guid_from_bytes(rguid),
    ) else {
        return E_INVALIDARG;
    };
    let mut st = CATEGORY_STATE.lock().unwrap();
    if !st.categories.contains(&(c, cat, g)) {
        st.categories.push((c, cat, g));
    }
    S_OK
}

unsafe extern "win64" fn cat_unregister_category(
    _this: *mut CategoryMgrObj,
    rclsid: *const u8,
    rcatid: *const u8,
    rguid: *const u8,
) -> HRESULT {
    let (Some(c), Some(cat), Some(g)) = (
        read_guid_from_bytes(rclsid),
        read_guid_from_bytes(rcatid),
        read_guid_from_bytes(rguid),
    ) else {
        return E_INVALIDARG;
    };
    let mut st = CATEGORY_STATE.lock().unwrap();
    st.categories.retain(|e| *e != (c, cat, g));
    S_OK
}

unsafe extern "win64" fn cat_enum_categories_in_item(
    _this: *mut CategoryMgrObj,
    rguid: *const u8,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    write_null_out(pp_enum);
    let Some(g) = read_guid_from_bytes(rguid) else {
        return E_INVALIDARG;
    };
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    let st = CATEGORY_STATE.lock().unwrap();
    let mut cats: Vec<Guid> = Vec::new();
    for &(_, cat, item) in &st.categories {
        if item == g && !cats.contains(&cat) {
            cats.push(cat);
        }
    }
    std::ptr::write_unaligned(pp_enum, new_enum_guid(cats, 0) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn cat_enum_items_in_category(
    _this: *mut CategoryMgrObj,
    rcatid: *const u8,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    write_null_out(pp_enum);
    let Some(cat) = read_guid_from_bytes(rcatid) else {
        return E_INVALIDARG;
    };
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    let st = CATEGORY_STATE.lock().unwrap();
    let mut items: Vec<Guid> = Vec::new();
    for &(_, c, item) in &st.categories {
        if c == cat && !items.contains(&item) {
            items.push(item);
        }
    }
    std::ptr::write_unaligned(pp_enum, new_enum_guid(items, 0) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn cat_find_closest_category(
    _this: *mut CategoryMgrObj,
    rguid: *const u8,
    pcatid: *mut u8,
    ppcatid_list: *const *const u8,
    ul_count: u32,
) -> HRESULT {
    write_guid(pcatid as *mut Guid, GUID_NULL);
    let Some(g) = read_guid_from_bytes(rguid) else {
        return E_INVALIDARG;
    };
    if pcatid.is_null() || (ul_count > 0 && ppcatid_list.is_null()) {
        return E_INVALIDARG;
    }
    let st = CATEGORY_STATE.lock().unwrap();
    if ul_count == 0 {
        if let Some(&(_, cat, _)) = st.categories.iter().find(|&&(_, _, item)| item == g) {
            write_guid(pcatid as *mut Guid, cat);
            return S_OK;
        }
        return S_FALSE;
    }
    for i in 0..ul_count as usize {
        let cand_ptr = std::ptr::read_unaligned(ppcatid_list.add(i));
        if let Some(cand) = read_guid_from_bytes(cand_ptr) {
            if st.categories.iter().any(|&(_, cat, item)| item == g && cat == cand) {
                write_guid(pcatid as *mut Guid, cand);
                return S_OK;
            }
        }
    }
    S_FALSE
}

unsafe extern "win64" fn cat_register_guid_desc(
    _this: *mut CategoryMgrObj,
    _rclsid: *const u8,
    rguid: *const u8,
    pch_desc: *const u16,
    cch: u32,
) -> HRESULT {
    let Some(g) = read_guid_from_bytes(rguid) else {
        return E_INVALIDARG;
    };
    let desc = read_wide_len(pch_desc, cch as usize);
    CATEGORY_STATE.lock().unwrap().descriptions.insert(g, desc);
    S_OK
}

unsafe extern "win64" fn cat_unregister_guid_desc(
    _this: *mut CategoryMgrObj,
    _rclsid: *const u8,
    rguid: *const u8,
) -> HRESULT {
    let Some(g) = read_guid_from_bytes(rguid) else {
        return E_INVALIDARG;
    };
    CATEGORY_STATE.lock().unwrap().descriptions.remove(&g);
    S_OK
}

unsafe extern "win64" fn cat_get_guid_desc(
    _this: *mut CategoryMgrObj,
    rguid: *const u8,
    pbstr_desc: *mut *mut u16,
) -> HRESULT {
    write_null_out(pbstr_desc);
    let Some(g) = read_guid_from_bytes(rguid) else {
        return E_INVALIDARG;
    };
    if pbstr_desc.is_null() {
        return E_INVALIDARG;
    }
    let found = CATEGORY_STATE.lock().unwrap().descriptions.get(&g).cloned();
    match found {
        Some(s) => {
            std::ptr::write_unaligned(pbstr_desc, alloc_bstr_str(&s));
            S_OK
        }
        None => E_FAIL,
    }
}

unsafe extern "win64" fn cat_register_guid_dword(
    _this: *mut CategoryMgrObj,
    _rclsid: *const u8,
    rguid: *const u8,
    dw: u32,
) -> HRESULT {
    let Some(g) = read_guid_from_bytes(rguid) else {
        return E_INVALIDARG;
    };
    CATEGORY_STATE.lock().unwrap().dwords.insert(g, dw);
    S_OK
}

unsafe extern "win64" fn cat_unregister_guid_dword(
    _this: *mut CategoryMgrObj,
    _rclsid: *const u8,
    rguid: *const u8,
) -> HRESULT {
    let Some(g) = read_guid_from_bytes(rguid) else {
        return E_INVALIDARG;
    };
    CATEGORY_STATE.lock().unwrap().dwords.remove(&g);
    S_OK
}

unsafe extern "win64" fn cat_get_guid_dword(
    _this: *mut CategoryMgrObj,
    rguid: *const u8,
    pdw: *mut u32,
) -> HRESULT {
    if !pdw.is_null() {
        std::ptr::write_unaligned(pdw, 0);
    }
    let Some(g) = read_guid_from_bytes(rguid) else {
        return E_INVALIDARG;
    };
    if pdw.is_null() {
        return E_INVALIDARG;
    }
    match CATEGORY_STATE.lock().unwrap().dwords.get(&g).copied() {
        Some(v) => {
            std::ptr::write_unaligned(pdw, v);
            S_OK
        }
        None => S_FALSE,
    }
}

unsafe extern "win64" fn cat_register_guid(
    _this: *mut CategoryMgrObj,
    rguid: *const u8,
    pguidatom: *mut TfGuidAtom,
) -> HRESULT {
    if !pguidatom.is_null() {
        std::ptr::write_unaligned(pguidatom, TF_INVALID_GUIDATOM);
    }
    let Some(g) = read_guid_from_bytes(rguid) else {
        return E_INVALIDARG;
    };
    if pguidatom.is_null() {
        return E_INVALIDARG;
    }
    let mut st = CATEGORY_STATE.lock().unwrap();
    let atom = if let Some(&existing) = st.guid_to_atom.get(&g) {
        existing
    } else {
        let mut a = st.next_atom;
        while a == TF_INVALID_GUIDATOM || st.atom_to_guid.contains_key(&a) {
            a = a.wrapping_add(1);
        }
        st.next_atom = a.wrapping_add(1);
        st.guid_to_atom.insert(g, a);
        st.atom_to_guid.insert(a, g);
        a
    };
    std::ptr::write_unaligned(pguidatom, atom);
    S_OK
}

unsafe extern "win64" fn cat_get_guid(
    _this: *mut CategoryMgrObj,
    guidatom: TfGuidAtom,
    pguid: *mut u8,
) -> HRESULT {
    if pguid.is_null() {
        return E_INVALIDARG;
    }
    write_guid(pguid as *mut Guid, GUID_NULL);
    match CATEGORY_STATE.lock().unwrap().atom_to_guid.get(&guidatom).copied() {
        Some(g) => {
            write_guid(pguid as *mut Guid, g);
            S_OK
        }
        None => S_FALSE,
    }
}

unsafe extern "win64" fn cat_is_equal_atom(
    _this: *mut CategoryMgrObj,
    guidatom: TfGuidAtom,
    rguid: *const u8,
    pf_equal: *mut BOOL,
) -> HRESULT {
    if !pf_equal.is_null() {
        std::ptr::write_unaligned(pf_equal, FALSE);
    }
    let Some(g) = read_guid_from_bytes(rguid) else {
        return E_INVALIDARG;
    };
    if pf_equal.is_null() {
        return E_INVALIDARG;
    }
    let eq = CATEGORY_STATE
        .lock()
        .unwrap()
        .atom_to_guid
        .get(&guidatom)
        .is_some_and(|found| *found == g);
    std::ptr::write_unaligned(pf_equal, if eq { TRUE } else { FALSE });
    S_OK
}

static ITF_CATEGORY_MGR_VTBL: ITfCategoryMgrVtbl = ITfCategoryMgrVtbl {
    query_interface: cat_qi,
    add_ref: com_add_ref::<CategoryMgrInner>,
    release: com_release::<CategoryMgrInner>,
    register_category: cat_register_category,
    unregister_category: cat_unregister_category,
    enum_categories_in_item: cat_enum_categories_in_item,
    enum_items_in_category: cat_enum_items_in_category,
    find_closest_category: cat_find_closest_category,
    register_guid_description: cat_register_guid_desc,
    unregister_guid_description: cat_unregister_guid_desc,
    get_guid_description: cat_get_guid_desc,
    register_guid_dword: cat_register_guid_dword,
    unregister_guid_dword: cat_unregister_guid_dword,
    get_guid_dword: cat_get_guid_dword,
    register_guid: cat_register_guid,
    get_guid: cat_get_guid,
    is_equal_tf_guid_atom: cat_is_equal_atom,
};

pub(super) fn new_category_mgr_obj() -> *mut CategoryMgrObj {
    register_vtable_ptrs(&[
        cat_qi as *const () as usize,
        com_add_ref::<CategoryMgrInner> as *const () as usize,
        com_release::<CategoryMgrInner> as *const () as usize,
        cat_register_category as *const () as usize,
        cat_unregister_category as *const () as usize,
        cat_enum_categories_in_item as *const () as usize,
        cat_enum_items_in_category as *const () as usize,
        cat_find_closest_category as *const () as usize,
        cat_register_guid_desc as *const () as usize,
        cat_unregister_guid_desc as *const () as usize,
        cat_get_guid_desc as *const () as usize,
        cat_register_guid_dword as *const () as usize,
        cat_unregister_guid_dword as *const () as usize,
        cat_get_guid_dword as *const () as usize,
        cat_register_guid as *const () as usize,
        cat_get_guid as *const () as usize,
        cat_is_equal_atom as *const () as usize,
    ]);
    ComObj::new(
        &ITF_CATEGORY_MGR_VTBL as *const ITfCategoryMgrVtbl as *const usize,
        CategoryMgrInner,
    )
}

// ---------------------------------------------------------------------------
// `ITfDisplayAttributeMgr` (6 slots) & `ITfDisplayAttributeInfo` (8 slots)
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct DisplayAttrEntry {
    guid: Guid,
    owner_clsid: Guid,
    desc: String,
    default_attr: TF_DISPLAYATTRIBUTE,
    current_attr: TF_DISPLAYATTRIBUTE,
}

struct DisplayAttrState {
    entries: Vec<DisplayAttrEntry>,
}

impl DisplayAttrState {
    fn new_default() -> Self {
        let input = TF_DISPLAYATTRIBUTE {
            cr_text: TF_DA_COLOR { ty: TF_CT_NONE, val: 0 },
            cr_bk: TF_DA_COLOR { ty: TF_CT_NONE, val: 0 },
            ls_style: TF_LS_DOT,
            f_bold_line: FALSE,
            cr_line: TF_DA_COLOR { ty: TF_CT_NONE, val: 0 },
            b_attr: TF_ATTR_INPUT,
        };
        let target_conv = TF_DISPLAYATTRIBUTE {
            cr_text: TF_DA_COLOR { ty: TF_CT_SYSCOLOR, val: 14 }, // COLOR_HIGHLIGHTTEXT
            cr_bk: TF_DA_COLOR { ty: TF_CT_SYSCOLOR, val: 13 },   // COLOR_HIGHLIGHT
            ls_style: TF_LS_NONE,
            f_bold_line: FALSE,
            cr_line: TF_DA_COLOR { ty: TF_CT_NONE, val: 0 },
            b_attr: TF_ATTR_TARGET_CONVERTED,
        };
        let converted = TF_DISPLAYATTRIBUTE {
            cr_text: TF_DA_COLOR { ty: TF_CT_NONE, val: 0 },
            cr_bk: TF_DA_COLOR { ty: TF_CT_NONE, val: 0 },
            ls_style: TF_LS_SOLID,
            f_bold_line: FALSE,
            cr_line: TF_DA_COLOR { ty: TF_CT_NONE, val: 0 },
            b_attr: TF_ATTR_CONVERTED,
        };
        Self {
            entries: vec![
                DisplayAttrEntry {
                    guid: GUID_DA_INPUT,
                    owner_clsid: CLSID_EN_US_KEYBOARD_TIP,
                    desc: String::from("Input"),
                    default_attr: input,
                    current_attr: input,
                },
                DisplayAttrEntry {
                    guid: GUID_DA_TARGET_CONVERTED,
                    owner_clsid: CLSID_EN_US_KEYBOARD_TIP,
                    desc: String::from("Target Converted"),
                    default_attr: target_conv,
                    current_attr: target_conv,
                },
                DisplayAttrEntry {
                    guid: GUID_DA_CONVERTED,
                    owner_clsid: CLSID_EN_US_KEYBOARD_TIP,
                    desc: String::from("Converted"),
                    default_attr: converted,
                    current_attr: converted,
                },
            ],
        }
    }
}

static DISPLAY_ATTR_STATE: LazyLock<Mutex<DisplayAttrState>> =
    LazyLock::new(|| Mutex::new(DisplayAttrState::new_default()));

#[repr(C)]
pub(super) struct ITfDisplayAttributeInfoVtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut DisplayAttrInfoObj, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut DisplayAttrInfoObj) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut DisplayAttrInfoObj) -> u32,
    pub(super) get_guid: unsafe extern "win64" fn(*mut DisplayAttrInfoObj, *mut u8) -> HRESULT,
    pub(super) get_description:
        unsafe extern "win64" fn(*mut DisplayAttrInfoObj, *mut *mut u16) -> HRESULT,
    pub(super) get_attribute_info:
        unsafe extern "win64" fn(*mut DisplayAttrInfoObj, *mut u8) -> HRESULT,
    pub(super) set_attribute_info:
        unsafe extern "win64" fn(*mut DisplayAttrInfoObj, *const u8) -> HRESULT,
    pub(super) reset: unsafe extern "win64" fn(*mut DisplayAttrInfoObj) -> HRESULT,
}
const _: () =
    assert!(std::mem::size_of::<ITfDisplayAttributeInfoVtbl>() == 8 * std::mem::size_of::<usize>());

pub(super) struct DisplayAttrInfoInner {
    guid: Guid,
}

pub(super) type DisplayAttrInfoObj = ComObj<DisplayAttrInfoInner>;

impl_single_qi!(dai_qi, DisplayAttrInfoObj, IID_ITFDISPLAYATTRIBUTEINFO);

unsafe extern "win64" fn dai_get_guid(this: *mut DisplayAttrInfoObj, pguid: *mut u8) -> HRESULT {
    if pguid.is_null() {
        return E_INVALIDARG;
    }
    write_guid(pguid as *mut Guid, ComObj::inner(this).guid);
    S_OK
}

unsafe extern "win64" fn dai_get_desc(
    this: *mut DisplayAttrInfoObj,
    pbstr: *mut *mut u16,
) -> HRESULT {
    if pbstr.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(pbstr);
    let guid = ComObj::inner(this).guid;
    let st = DISPLAY_ATTR_STATE.lock().unwrap();
    if let Some(e) = st.entries.iter().find(|e| e.guid == guid) {
        std::ptr::write_unaligned(pbstr, alloc_bstr_str(&e.desc));
        S_OK
    } else {
        E_FAIL
    }
}

unsafe extern "win64" fn dai_get_attr(this: *mut DisplayAttrInfoObj, pda: *mut u8) -> HRESULT {
    if pda.is_null() {
        return E_INVALIDARG;
    }
    let guid = ComObj::inner(this).guid;
    let st = DISPLAY_ATTR_STATE.lock().unwrap();
    if let Some(e) = st.entries.iter().find(|e| e.guid == guid) {
        std::ptr::write_unaligned(pda as *mut TF_DISPLAYATTRIBUTE, e.current_attr);
        S_OK
    } else {
        E_FAIL
    }
}

unsafe extern "win64" fn dai_set_attr(this: *mut DisplayAttrInfoObj, pda: *const u8) -> HRESULT {
    if pda.is_null() {
        return E_INVALIDARG;
    }
    let new_attr = std::ptr::read_unaligned(pda as *const TF_DISPLAYATTRIBUTE);
    let guid = ComObj::inner(this).guid;
    let mut st = DISPLAY_ATTR_STATE.lock().unwrap();
    if let Some(e) = st.entries.iter_mut().find(|e| e.guid == guid) {
        e.current_attr = new_attr;
        S_OK
    } else {
        E_FAIL
    }
}

unsafe extern "win64" fn dai_reset(this: *mut DisplayAttrInfoObj) -> HRESULT {
    let guid = ComObj::inner(this).guid;
    let mut st = DISPLAY_ATTR_STATE.lock().unwrap();
    if let Some(e) = st.entries.iter_mut().find(|e| e.guid == guid) {
        e.current_attr = e.default_attr;
        S_OK
    } else {
        E_FAIL
    }
}

static ITF_DISPLAY_ATTR_INFO_VTBL: ITfDisplayAttributeInfoVtbl = ITfDisplayAttributeInfoVtbl {
    query_interface: dai_qi,
    add_ref: com_add_ref::<DisplayAttrInfoInner>,
    release: com_release::<DisplayAttrInfoInner>,
    get_guid: dai_get_guid,
    get_description: dai_get_desc,
    get_attribute_info: dai_get_attr,
    set_attribute_info: dai_set_attr,
    reset: dai_reset,
};

fn new_display_attr_info_obj(guid: Guid) -> *mut DisplayAttrInfoObj {
    register_vtable_ptrs(&[
        dai_qi as *const () as usize,
        com_add_ref::<DisplayAttrInfoInner> as *const () as usize,
        com_release::<DisplayAttrInfoInner> as *const () as usize,
        dai_get_guid as *const () as usize,
        dai_get_desc as *const () as usize,
        dai_get_attr as *const () as usize,
        dai_set_attr as *const () as usize,
        dai_reset as *const () as usize,
    ]);
    ComObj::new(
        &ITF_DISPLAY_ATTR_INFO_VTBL as *const ITfDisplayAttributeInfoVtbl as *const usize,
        DisplayAttrInfoInner { guid },
    )
}

#[repr(C)]
pub(super) struct ITfDisplayAttributeMgrVtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut DisplayAttrMgrObj, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut DisplayAttrMgrObj) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut DisplayAttrMgrObj) -> u32,
    pub(super) on_update_info: unsafe extern "win64" fn(*mut DisplayAttrMgrObj) -> HRESULT,
    pub(super) enum_display_attribute_info:
        unsafe extern "win64" fn(*mut DisplayAttrMgrObj, *mut *mut c_void) -> HRESULT,
    pub(super) get_display_attribute_info: unsafe extern "win64" fn(
        *mut DisplayAttrMgrObj,
        *const u8,
        *mut *mut c_void,
        *mut u8,
    ) -> HRESULT,
}
const _: () =
    assert!(std::mem::size_of::<ITfDisplayAttributeMgrVtbl>() == 6 * std::mem::size_of::<usize>());

pub(super) struct DisplayAttrMgrInner;

pub(super) type DisplayAttrMgrObj = ComObj<DisplayAttrMgrInner>;

impl_single_qi!(dam_qi, DisplayAttrMgrObj, IID_ITFDISPLAYATTRIBUTEMGR);

unsafe extern "win64" fn dam_on_update_info(_this: *mut DisplayAttrMgrObj) -> HRESULT {
    S_OK
}

unsafe extern "win64" fn dam_enum_info(
    _this: *mut DisplayAttrMgrObj,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    let guids: Vec<Guid> =
        DISPLAY_ATTR_STATE.lock().unwrap().entries.iter().map(|e| e.guid).collect();
    let ptrs: Vec<usize> =
        guids.into_iter().map(|g| new_display_attr_info_obj(g) as usize).collect();
    let en = new_enum_tf_com(IID_IENUMTFDISPLAYATTRIBUTEINFO, ptrs, 0);
    std::ptr::write_unaligned(pp_enum, en as *mut c_void);
    S_OK
}

unsafe extern "win64" fn dam_get_info(
    _this: *mut DisplayAttrMgrObj,
    guid: *const u8,
    pp_info: *mut *mut c_void,
    pclsid_owner: *mut u8,
) -> HRESULT {
    write_null_out(pp_info);
    write_guid(pclsid_owner as *mut Guid, GUID_NULL);
    let Some(g) = read_guid_from_bytes(guid) else {
        return E_INVALIDARG;
    };
    if pp_info.is_null() {
        return E_INVALIDARG;
    }
    let owner = DISPLAY_ATTR_STATE
        .lock()
        .unwrap()
        .entries
        .iter()
        .find(|e| e.guid == g)
        .map(|e| e.owner_clsid);
    match owner {
        Some(clsid) => {
            let info = new_display_attr_info_obj(g);
            std::ptr::write_unaligned(pp_info, info as *mut c_void);
            write_guid(pclsid_owner as *mut Guid, clsid);
            S_OK
        }
        None => E_INVALIDARG,
    }
}

static ITF_DISPLAY_ATTR_MGR_VTBL: ITfDisplayAttributeMgrVtbl = ITfDisplayAttributeMgrVtbl {
    query_interface: dam_qi,
    add_ref: com_add_ref::<DisplayAttrMgrInner>,
    release: com_release::<DisplayAttrMgrInner>,
    on_update_info: dam_on_update_info,
    enum_display_attribute_info: dam_enum_info,
    get_display_attribute_info: dam_get_info,
};

pub(super) fn new_display_attr_mgr_obj() -> *mut DisplayAttrMgrObj {
    register_vtable_ptrs(&[
        dam_qi as *const () as usize,
        com_add_ref::<DisplayAttrMgrInner> as *const () as usize,
        com_release::<DisplayAttrMgrInner> as *const () as usize,
        dam_on_update_info as *const () as usize,
        dam_enum_info as *const () as usize,
        dam_get_info as *const () as usize,
    ]);
    ComObj::new(
        &ITF_DISPLAY_ATTR_MGR_VTBL as *const ITfDisplayAttributeMgrVtbl as *const usize,
        DisplayAttrMgrInner,
    )
}

// ---------------------------------------------------------------------------
// `ITfLangBarMgr` (12 slots — ctfutb.h)
// ---------------------------------------------------------------------------

#[repr(C)]
pub(super) struct ITfLangBarMgrVtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut LangBarMgrObj, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut LangBarMgrObj) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut LangBarMgrObj) -> u32,
    pub(super) advise_event_sink:
        unsafe extern "win64" fn(*mut LangBarMgrObj, *mut c_void, HWND, u32, *mut u32) -> HRESULT,
    pub(super) unadvise_event_sink: unsafe extern "win64" fn(*mut LangBarMgrObj, u32) -> HRESULT,
    pub(super) get_thread_marshal_interface: unsafe extern "win64" fn(
        *mut LangBarMgrObj,
        u32,
        u32,
        *const u8,
        *mut *mut c_void,
    ) -> HRESULT,
    pub(super) get_thread_lang_bar_item_mgr:
        unsafe extern "win64" fn(*mut LangBarMgrObj, u32, *mut *mut c_void, *mut u32) -> HRESULT,
    pub(super) get_input_processor_profiles:
        unsafe extern "win64" fn(*mut LangBarMgrObj, u32, *mut *mut c_void, *mut u32) -> HRESULT,
    pub(super) restore_last_focus:
        unsafe extern "win64" fn(*mut LangBarMgrObj, *mut u32, BOOL) -> HRESULT,
    pub(super) set_modal_input:
        unsafe extern "win64" fn(*mut LangBarMgrObj, *mut c_void, u32, u32) -> HRESULT,
    pub(super) show_floating: unsafe extern "win64" fn(*mut LangBarMgrObj, u32) -> HRESULT,
    pub(super) get_show_floating_status:
        unsafe extern "win64" fn(*mut LangBarMgrObj, *mut u32) -> HRESULT,
}
const _: () =
    assert!(std::mem::size_of::<ITfLangBarMgrVtbl>() == 12 * std::mem::size_of::<usize>());

struct LangBarState {
    floating_flags: u32,
    next_cookie: u32,
    sinks: HashMap<u32, usize>,
}

pub(super) struct LangBarMgrInner {
    state: Mutex<LangBarState>,
}

impl Drop for LangBarMgrInner {
    fn drop(&mut self) {
        let sinks: Vec<*mut c_void> = self
            .state
            .lock()
            .unwrap()
            .sinks
            .drain()
            .map(|(_, p)| p as *mut c_void)
            .collect();
        unsafe { release_all_ptrs(sinks) };
    }
}

pub(super) type LangBarMgrObj = ComObj<LangBarMgrInner>;

impl_single_qi!(lbm_qi, LangBarMgrObj, IID_ITFLANGBARMGR);

unsafe extern "win64" fn lbm_advise_event_sink(
    this: *mut LangBarMgrObj,
    p_sink: *mut c_void,
    _hwnd: HWND,
    _dw_flags: u32,
    pdw_cookie: *mut u32,
) -> HRESULT {
    if !pdw_cookie.is_null() {
        std::ptr::write_unaligned(pdw_cookie, TF_INVALID_COOKIE);
    }
    if p_sink.is_null() || pdw_cookie.is_null() {
        return E_INVALIDARG;
    }
    com_call_add_ref(p_sink);
    let cookie = {
        let mut st = ComObj::inner(this).state.lock().unwrap();
        let mut c = st.next_cookie;
        while c == 0 || c == TF_INVALID_COOKIE || st.sinks.contains_key(&c) {
            c = c.wrapping_add(1);
        }
        st.next_cookie = c.wrapping_add(1);
        st.sinks.insert(c, p_sink as usize);
        c
    };
    std::ptr::write_unaligned(pdw_cookie, cookie);
    S_OK
}

unsafe extern "win64" fn lbm_unadvise_event_sink(
    this: *mut LangBarMgrObj,
    dw_cookie: u32,
) -> HRESULT {
    let removed = ComObj::inner(this).state.lock().unwrap().sinks.remove(&dw_cookie);
    match removed {
        Some(p) => {
            com_call_release(p as *mut c_void);
            S_OK
        }
        None => CONNECT_E_NOCONNECTION,
    }
}

unsafe extern "win64" fn lbm_get_thread_marshal_interface(
    _this: *mut LangBarMgrObj,
    _dw_thread_id: u32,
    _dw_type: u32,
    _riid: *const u8,
    ppunk: *mut *mut c_void,
) -> HRESULT {
    write_null_out(ppunk);
    E_NOTIMPL
}

unsafe extern "win64" fn lbm_get_thread_item_mgr(
    _this: *mut LangBarMgrObj,
    _dw_thread_id: u32,
    pplbie: *mut *mut c_void,
    pdw_threadid: *mut u32,
) -> HRESULT {
    write_null_out(pplbie);
    if !pdw_threadid.is_null() {
        std::ptr::write_unaligned(pdw_threadid, current_tid());
    }
    E_NOTIMPL
}

unsafe extern "win64" fn lbm_get_input_processor_profiles(
    _this: *mut LangBarMgrObj,
    dw_thread_id: u32,
    ppaip: *mut *mut c_void,
    pdw_threadid: *mut u32,
) -> HRESULT {
    if ppaip.is_null() {
        return E_INVALIDARG;
    }
    let prof = new_input_processor_profiles_obj();
    std::ptr::write_unaligned(ppaip, prof as *mut c_void);
    if !pdw_threadid.is_null() {
        let tid = if dw_thread_id == 0 { current_tid() } else { dw_thread_id };
        std::ptr::write_unaligned(pdw_threadid, tid);
    }
    S_OK
}

unsafe extern "win64" fn lbm_restore_last_focus(
    _this: *mut LangBarMgrObj,
    pdw_thread_id: *mut u32,
    _f_prev: BOOL,
) -> HRESULT {
    if !pdw_thread_id.is_null() {
        std::ptr::write_unaligned(pdw_thread_id, current_tid());
    }
    S_OK
}

unsafe extern "win64" fn lbm_set_modal_input(
    _this: *mut LangBarMgrObj,
    _p_sink: *mut c_void,
    _dw_thread_id: u32,
    _dw_flags: u32,
) -> HRESULT {
    S_OK
}

unsafe extern "win64" fn lbm_show_floating(this: *mut LangBarMgrObj, dw_flags: u32) -> HRESULT {
    ComObj::inner(this).state.lock().unwrap().floating_flags = dw_flags;
    S_OK
}

unsafe extern "win64" fn lbm_get_show_floating_status(
    this: *mut LangBarMgrObj,
    pdw_flags: *mut u32,
) -> HRESULT {
    if pdw_flags.is_null() {
        return E_INVALIDARG;
    }
    let flags = ComObj::inner(this).state.lock().unwrap().floating_flags;
    std::ptr::write_unaligned(pdw_flags, flags);
    S_OK
}

static ITF_LANG_BAR_MGR_VTBL: ITfLangBarMgrVtbl = ITfLangBarMgrVtbl {
    query_interface: lbm_qi,
    add_ref: com_add_ref::<LangBarMgrInner>,
    release: com_release::<LangBarMgrInner>,
    advise_event_sink: lbm_advise_event_sink,
    unadvise_event_sink: lbm_unadvise_event_sink,
    get_thread_marshal_interface: lbm_get_thread_marshal_interface,
    get_thread_lang_bar_item_mgr: lbm_get_thread_item_mgr,
    get_input_processor_profiles: lbm_get_input_processor_profiles,
    restore_last_focus: lbm_restore_last_focus,
    set_modal_input: lbm_set_modal_input,
    show_floating: lbm_show_floating,
    get_show_floating_status: lbm_get_show_floating_status,
};

pub(super) fn new_lang_bar_mgr_obj() -> *mut LangBarMgrObj {
    register_vtable_ptrs(&[
        lbm_qi as *const () as usize,
        com_add_ref::<LangBarMgrInner> as *const () as usize,
        com_release::<LangBarMgrInner> as *const () as usize,
        lbm_advise_event_sink as *const () as usize,
        lbm_unadvise_event_sink as *const () as usize,
        lbm_get_thread_marshal_interface as *const () as usize,
        lbm_get_thread_item_mgr as *const () as usize,
        lbm_get_input_processor_profiles as *const () as usize,
        lbm_restore_last_focus as *const () as usize,
        lbm_set_modal_input as *const () as usize,
        lbm_show_floating as *const () as usize,
        lbm_get_show_floating_status as *const () as usize,
    ]);
    ComObj::new(
        &ITF_LANG_BAR_MGR_VTBL as *const ITfLangBarMgrVtbl as *const usize,
        LangBarMgrInner {
            state: Mutex::new(LangBarState {
                floating_flags: 0x0008, // TF_SFT_HIDDEN
                next_cookie: 1,
                sinks: HashMap::new(),
            }),
        },
    )
}
