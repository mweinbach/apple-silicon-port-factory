//! CRT locale & multibyte codepage (`setlocale`, `_wsetlocale`, `localeconv`,
//! `_create_locale`, `_setmbcp`, `_getmbcp`), backed by real Windows/ICU locale
//! and codepage tables via [`crate::kernel32::string`].

use super::errno::{self, EINVAL};
use crate::kernel32::string::{self as kstr, ResolvedLocale};
use crate::strings::*;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};

const LC_ALL: i32 = 0;
const LC_COLLATE: i32 = 1;
const LC_CTYPE: i32 = 2;
const LC_MONETARY: i32 = 3;
const LC_NUMERIC: i32 = 4;
const LC_TIME: i32 = 5;

const _MB_CP_SBCS: i32 = 0;
const _MB_CP_OEM: i32 = -2;
const _MB_CP_ANSI: i32 = -3;
const _MB_CP_LOCALE: i32 = -4;

static C_LOCALE: AtomicBool = AtomicBool::new(true);
static CTYPE_CP: AtomicU32 = AtomicU32::new(0);
static COLLATE_CP: AtomicU32 = AtomicU32::new(0);
static MB_CODEPAGE: AtomicI32 = AtomicI32::new(ACP as i32);

pub fn c_locale_active() -> bool {
    C_LOCALE.load(Ordering::Relaxed)
}

/// Multibyte code page of the current CRT `LC_CTYPE` locale (0 = "C": bytes map 1:1).
pub fn mb_cp() -> u32 {
    CTYPE_CP.load(Ordering::Relaxed)
}

/// Multibyte code page for an optional `_locale_t` (falling back to the current global `LC_CTYPE`).
pub(super) unsafe fn locale_obj_cp(l: *mut c_void) -> u32 {
    if l.is_null() {
        mb_cp()
    } else {
        let obj = &*(l as *const LocaleObj);
        if obj.entry.is_c { 0 } else { obj.entry.cp }
    }
}

/// `(cp, icu_locale)` for the active global `LC_CTYPE` or an explicit `_locale_t`.
pub(super) unsafe fn ctype_cp_and_icu(l: *mut c_void) -> (u32, String) {
    let (is_c, cp, bcp47) = if l.is_null() {
        let st = global_state().lock().unwrap();
        let e = &st.cats[(LC_CTYPE - 1) as usize];
        (e.is_c, e.cp, e.bcp47.clone())
    } else {
        let obj = &*(l as *const LocaleObj);
        (obj.entry.is_c, obj.entry.cp, obj.entry.bcp47.clone())
    };
    if is_c || cp == 0 {
        return (0, String::new());
    }
    let icu_id = kstr::resolve_locale_tag(&bcp47).map(|r| r.icu_id).unwrap_or_default();
    (cp, icu_id)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LocaleEntry {
    is_c: bool,
    bcp47: String,
    crt_name: String,
    cp: u32,
}

impl LocaleEntry {
    fn c_locale() -> Self {
        Self {
            is_c: true,
            bcp47: String::new(),
            crt_name: "C".into(),
            cp: 0,
        }
    }
    fn user_default() -> Self {
        Self {
            is_c: false,
            bcp47: "en-US".into(),
            crt_name: "English_United States.utf8".into(),
            cp: 65001,
        }
    }
    fn max_char_size(&self) -> i32 {
        if self.is_c || self.cp == 0 {
            1
        } else {
            kstr::cp_max_char_size(self.cp) as i32
        }
    }
}

struct LocaleState {
    /// Categories 0..5 correspond to LC_COLLATE(1)..LC_TIME(5).
    cats: [LocaleEntry; 5],
}

fn global_state() -> &'static Mutex<LocaleState> {
    static ST: OnceLock<Mutex<LocaleState>> = OnceLock::new();
    ST.get_or_init(|| {
        Mutex::new(LocaleState {
            cats: [
                LocaleEntry::c_locale(),
                LocaleEntry::c_locale(),
                LocaleEntry::c_locale(),
                LocaleEntry::c_locale(),
                LocaleEntry::c_locale(),
            ],
        })
    })
}

fn sync_atomics(st: &LocaleState) {
    let ctype = &st.cats[(LC_CTYPE - 1) as usize];
    let collate = &st.cats[(LC_COLLATE - 1) as usize];
    C_LOCALE.store(ctype.is_c, Ordering::Relaxed);
    CTYPE_CP.store(ctype.cp, Ordering::Relaxed);
    COLLATE_CP.store(collate.cp, Ordering::Relaxed);
    let icu_id = if ctype.is_c || ctype.cp == 0 {
        String::new()
    } else {
        kstr::resolve_locale_tag(&ctype.bcp47).map(|r| r.icu_id).unwrap_or_default()
    };
    super::string::refresh_global_ctype_tables(if ctype.is_c { 0 } else { ctype.cp }, &icu_id);
}

fn intern_bytes(bytes: &[u8]) -> *mut u8 {
    struct SyncMap(Mutex<HashMap<Vec<u8>, usize>>);
    unsafe impl Sync for SyncMap {}
    static CACHE: OnceLock<SyncMap> = OnceLock::new();
    let m = &CACHE.get_or_init(|| SyncMap(Mutex::new(HashMap::new()))).0;
    let mut g = m.lock().unwrap();
    if let Some(&p) = g.get(bytes) {
        return p as *mut u8;
    }
    let mut v = bytes.to_vec();
    v.push(0);
    let p = Box::leak(v.into_boxed_slice()).as_mut_ptr();
    g.insert(bytes.to_vec(), p as usize);
    p
}

fn intern_ansi(s: &str) -> *mut u8 {
    intern_bytes(s.as_bytes())
}

fn intern_in_cp(s: &str, cp: u32) -> *const u8 {
    if cp == 0 || cp == 65001 {
        return intern_bytes(s.as_bytes());
    }
    let w = to_wide(s);
    match kstr::encode_utf16_to_cp(cp, &w, Some(b'?')) {
        Ok((bytes, _)) => intern_bytes(&bytes),
        Err(_) => intern_bytes(s.as_bytes()),
    }
}

fn intern_wide(s: &str) -> *mut u16 {
    struct SyncMap(Mutex<HashMap<String, usize>>);
    unsafe impl Sync for SyncMap {}
    static CACHE: OnceLock<SyncMap> = OnceLock::new();
    let m = &CACHE.get_or_init(|| SyncMap(Mutex::new(HashMap::new()))).0;
    let mut g = m.lock().unwrap();
    if let Some(&p) = g.get(s) {
        return p as *mut u16;
    }
    let p = leak_wide_z(s);
    g.insert(s.to_string(), p as usize);
    p
}

fn parse_codepage_suffix(s: &str) -> Option<u32> {
    let u = s.trim().to_ascii_uppercase();
    match u.as_str() {
        "UTF8" | "UTF-8" | "ACP" | "OCP" => Some(65001),
        _ => {
            let n = u.parse::<u32>().ok()?;
            if kstr::valid_cp(n) {
                Some(kstr::resolve_cp(n))
            } else {
                None
            }
        }
    }
}

/// Map legacy MSVC CRT language/country names and 3-letter abbreviations to BCP-47 tags.
fn legacy_crt_to_bcp47(lang_part: &str) -> Option<&'static str> {
    let lc = lang_part.trim().to_ascii_lowercase().replace('-', "_");
    let (lang, country) = match lc.split_once('_') {
        Some((l, c)) => (l.trim(), Some(c.trim())),
        None => (lc.as_str(), None),
    };
    Some(match (lang, country) {
        ("english" | "enu", None | Some("united states" | "usa" | "us")) => "en-US",
        ("english" | "eng", Some("united kingdom" | "uk" | "gbr" | "britain" | "great britain")) => "en-GB",
        ("english" | "enc", Some("canada" | "can")) => "en-CA",
        ("english" | "ena", Some("australia" | "aus")) => "en-AU",
        ("english", Some("ireland" | "irl")) => "en-IE",
        ("english", Some("new zealand" | "nzl")) => "en-NZ",
        ("german" | "deu" | "ger", None | Some("germany" | "deu" | "de")) => "de-DE",
        ("german" | "dea", Some("austria" | "aut")) => "de-AT",
        ("german" | "des", Some("switzerland" | "che" | "swiss")) => "de-CH",
        ("french" | "fra" | "fre", None | Some("france" | "fra" | "fr")) => "fr-FR",
        ("french" | "frc", Some("canada" | "can")) => "fr-CA",
        ("french" | "frs", Some("switzerland" | "che")) => "fr-CH",
        ("french" | "frb", Some("belgium" | "bel")) => "fr-BE",
        ("japanese" | "jpn", None | Some("japan" | "jpn" | "jp")) => "ja-JP",
        ("chinese" | "chs" | "chinese-simplified" | "chinese_simplified", None | Some("china" | "chn" | "cn" | "prc" | "simplified")) => "zh-CN",
        ("chinese" | "cht" | "chinese-traditional" | "chinese_traditional", Some("taiwan" | "twn" | "tw" | "traditional")) => "zh-TW",
        ("chinese", Some("hong kong" | "hkg" | "hk")) => "zh-HK",
        ("korean" | "kor", None | Some("korea" | "kor" | "kr")) => "ko-KR",
        ("spanish" | "esp" | "esn", None | Some("spain" | "esp" | "es")) => "es-ES",
        ("spanish" | "esm", Some("mexico" | "mex" | "mx")) => "es-MX",
        ("italian" | "ita", None | Some("italy" | "ita" | "it")) => "it-IT",
        ("portuguese" | "ptb", None | Some("brazil" | "bra" | "br")) => "pt-BR",
        ("portuguese" | "ptg", Some("portugal" | "prt" | "pt")) => "pt-PT",
        ("russian" | "rus", None | Some("russia" | "rus" | "ru")) => "ru-RU",
        ("dutch" | "nld", None | Some("netherlands" | "nld" | "nl" | "holland")) => "nl-NL",
        ("swedish" | "sve", None | Some("sweden" | "swe" | "se")) => "sv-SE",
        ("finnish" | "fin", None | Some("finland" | "fin" | "fi")) => "fi-FI",
        ("danish" | "dan", None | Some("denmark" | "dnk" | "dk")) => "da-DK",
        ("norwegian" | "nor" | "norwegian-bokmal", None | Some("norway" | "nor" | "no")) => "nb-NO",
        ("polish" | "plk", None | Some("poland" | "pol" | "pl")) => "pl-PL",
        ("czech" | "csy", None | Some("czechia" | "czech republic" | "cze" | "cz")) => "cs-CZ",
        ("hungarian" | "hun", None | Some("hungary" | "hun" | "hu")) => "hu-HU",
        ("turkish" | "trk", None | Some("turkey" | "tur" | "tr")) => "tr-TR",
        ("greek" | "ell", None | Some("greece" | "grc" | "gr")) => "el-GR",
        ("hebrew" | "heb", None | Some("israel" | "isr" | "il")) => "he-IL",
        ("arabic" | "ara", None | Some("saudi arabia" | "sau" | "sa")) => "ar-SA",
        ("thai" | "tha", None | Some("thailand" | "tha" | "th")) => "th-TH",
        ("ukrainian" | "ukr", None | Some("ukraine" | "ukr" | "ua")) => "uk-UA",
        ("vietnamese" | "vit", None | Some("vietnam" | "vnm" | "vn")) => "vi-VN",
        ("romanian" | "rom", None | Some("romania" | "rou" | "ro")) => "ro-RO",
        ("bulgarian" | "bgr", None | Some("bulgaria" | "bgr" | "bg")) => "bg-BG",
        ("croatian" | "hrv", None | Some("croatia" | "hrv" | "hr")) => "hr-HR",
        ("slovak" | "sky", None | Some("slovakia" | "svk" | "sk")) => "sk-SK",
        ("indonesian" | "ind", None | Some("indonesia" | "idn" | "id")) => "id-ID",
        _ => return None,
    })
}

fn format_crt_name(loc: &ResolvedLocale, cp: u32) -> String {
    let lang = kstr::locale_info_for(loc, 0x1001).unwrap_or_else(|| loc.bcp47.clone());
    let country = kstr::locale_info_for(loc, 0x1002).unwrap_or_default();
    let cp_str = if cp == 65001 { "utf8".to_string() } else { cp.to_string() };
    if country.is_empty() {
        format!("{lang}.{cp_str}")
    } else {
        format!("{lang}_{country}.{cp_str}")
    }
}

fn parse_single_locale(name: &str) -> Option<LocaleEntry> {
    let trimmed = name.trim();
    if trimmed.eq_ignore_ascii_case("C") || trimmed.eq_ignore_ascii_case("POSIX") {
        return Some(LocaleEntry::c_locale());
    }
    if trimmed.is_empty() {
        return Some(LocaleEntry::user_default());
    }
    // `.codepage` shorthand (e.g. `.UTF8`, `.1252`, `.ACP`, `.OCP`, `.932`)
    if let Some(cp_part) = trimmed.strip_prefix('.') {
        let cp = parse_codepage_suffix(cp_part)?;
        let loc = kstr::resolve_locale_tag("en-US")?;
        return Some(LocaleEntry {
            is_c: false,
            bcp47: loc.bcp47.clone(),
            crt_name: format_crt_name(&loc, cp),
            cp,
        });
    }
    // Split optional `.codepage` suffix (strip any `@modifier` if present).
    let (base, cp_opt) = match trimmed.split_once('.') {
        Some((b, c)) => {
            let c_clean = c.split('@').next().unwrap_or(c);
            (b.trim(), Some(parse_codepage_suffix(c_clean)?))
        }
        None => (trimmed.split('@').next().unwrap_or(trimmed).trim(), None),
    };
    if base.eq_ignore_ascii_case("C") || base.eq_ignore_ascii_case("POSIX") {
        let cp = cp_opt.unwrap_or(0);
        if cp == 0 {
            return Some(LocaleEntry::c_locale());
        }
        return Some(LocaleEntry {
            is_c: false,
            bcp47: "en-US".into(),
            crt_name: format!("C.{cp}"),
            cp,
        });
    }
    let bcp47_candidate = legacy_crt_to_bcp47(base).unwrap_or(base);
    let loc = kstr::resolve_locale_tag(bcp47_candidate)?;
    if loc.invariant {
        return Some(LocaleEntry::c_locale());
    }
    let default_cp = kstr::locale_info_for(&loc, 0x1004)
        .and_then(|s| s.parse::<u32>().ok())
        .filter(|&c| c != 0 && kstr::valid_cp(c))
        .unwrap_or(65001);
    let cp = cp_opt.unwrap_or(default_cp);
    Some(LocaleEntry {
        is_c: false,
        bcp47: loc.bcp47.clone(),
        crt_name: format_crt_name(&loc, cp),
        cp,
    })
}

fn format_current_name(st: &LocaleState, cat: i32) -> String {
    if cat == LC_ALL {
        let first = &st.cats[0].crt_name;
        if st.cats.iter().all(|c| &c.crt_name == first) {
            first.clone()
        } else {
            format!(
                "LC_COLLATE={};LC_CTYPE={};LC_MONETARY={};LC_NUMERIC={};LC_TIME={}",
                st.cats[0].crt_name,
                st.cats[1].crt_name,
                st.cats[2].crt_name,
                st.cats[3].crt_name,
                st.cats[4].crt_name,
            )
        }
    } else {
        st.cats[(cat - 1) as usize].crt_name.clone()
    }
}

fn apply_setlocale(cat: i32, name: Option<&str>) -> Option<String> {
    if !(LC_ALL..=LC_TIME).contains(&cat) {
        return None;
    }
    let mut st = global_state().lock().unwrap();
    if let Some(n) = name {
        if cat == LC_ALL && n.contains(';') && n.contains('=') {
            // Composite string: `LC_COLLATE=...;LC_CTYPE=...;...`
            let mut next = st.cats.clone();
            for part in n.split(';') {
                let (k, v) = part.split_once('=')?;
                let idx = match k.trim() {
                    "LC_COLLATE" => 0,
                    "LC_CTYPE" => 1,
                    "LC_MONETARY" => 2,
                    "LC_NUMERIC" => 3,
                    "LC_TIME" => 4,
                    _ => return None,
                };
                next[idx] = parse_single_locale(v)?;
            }
            st.cats = next;
        } else {
            let entry = parse_single_locale(n)?;
            if cat == LC_ALL {
                for slot in &mut st.cats {
                    *slot = entry.clone();
                }
            } else {
                st.cats[(cat - 1) as usize] = entry;
            }
        }
        sync_atomics(&st);
    }
    Some(format_current_name(&st, cat))
}

pub unsafe extern "win64" fn setlocale(cat: i32, name: *const u8) -> *mut u8 {
    let req = from_ansi_opt(name);
    match apply_setlocale(cat, req.as_deref()) {
        Some(res) => {
            log::debug!("setlocale({cat}, {req:?}) -> {res:?}");
            intern_ansi(&res)
        }
        None => std::ptr::null_mut(),
    }
}

pub unsafe extern "win64" fn _wsetlocale(cat: i32, name: *const u16) -> *mut u16 {
    let req = from_wide_opt(name);
    match apply_setlocale(cat, req.as_deref()) {
        Some(res) => {
            log::debug!("_wsetlocale({cat}, {req:?}) -> {res:?}");
            intern_wide(&res)
        }
        None => std::ptr::null_mut(),
    }
}

#[repr(C)]
pub struct Lconv {
    pub decimal_point: *const u8,
    pub thousands_sep: *const u8,
    pub grouping: *const u8,
    pub int_curr_symbol: *const u8,
    pub currency_symbol: *const u8,
    pub mon_decimal_point: *const u8,
    pub mon_thousands_sep: *const u8,
    pub mon_grouping: *const u8,
    pub positive_sign: *const u8,
    pub negative_sign: *const u8,
    pub int_frac_digits: u8,
    pub frac_digits: u8,
    pub p_cs_precedes: u8,
    pub p_sep_by_space: u8,
    pub n_cs_precedes: u8,
    pub n_sep_by_space: u8,
    pub p_sign_posn: u8,
    pub n_sign_posn: u8,
    pub w_decimal_point: *const u16,
    pub w_thousands_sep: *const u16,
    pub w_int_curr_symbol: *const u16,
    pub w_currency_symbol: *const u16,
    pub w_mon_decimal_point: *const u16,
    pub w_mon_thousands_sep: *const u16,
    pub w_positive_sign: *const u16,
    pub w_negative_sign: *const u16,
}
unsafe impl Sync for Lconv {}
unsafe impl Send for Lconv {}

fn lconv_c() -> &'static Lconv {
    static L: OnceLock<Lconv> = OnceLock::new();
    L.get_or_init(|| {
        let e = leak_ansi_z("");
        let we = leak_wide_z("");
        Lconv {
            decimal_point: leak_ansi_z("."),
            thousands_sep: e,
            grouping: e,
            int_curr_symbol: e,
            currency_symbol: e,
            mon_decimal_point: e,
            mon_thousands_sep: e,
            mon_grouping: e,
            positive_sign: e,
            negative_sign: e,
            int_frac_digits: 127,
            frac_digits: 127,
            p_cs_precedes: 127,
            p_sep_by_space: 127,
            n_cs_precedes: 127,
            n_sep_by_space: 127,
            p_sign_posn: 127,
            n_sign_posn: 127,
            w_decimal_point: leak_wide_z("."),
            w_thousands_sep: we,
            w_int_curr_symbol: we,
            w_currency_symbol: we,
            w_mon_decimal_point: we,
            w_mon_thousands_sep: we,
            w_positive_sign: we,
            w_negative_sign: we,
        }
    })
}

fn grouping_bytes(s: &str) -> String {
    let mut out = String::new();
    for p in s.split(';') {
        if let Ok(n) = p.trim().parse::<u8>() {
            if n == 0 {
                break;
            }
            out.push(n as char);
        }
    }
    out
}

fn build_lconv(num: &LocaleEntry, mon: &LocaleEntry) -> *const Lconv {
    if num.is_c && mon.is_c {
        return lconv_c();
    }
    let key = format!("{}|{}|{}|{}", num.bcp47, num.cp, mon.bcp47, mon.cp);
    struct SyncMap(Mutex<HashMap<String, usize>>);
    unsafe impl Sync for SyncMap {}
    static CACHE: OnceLock<SyncMap> = OnceLock::new();
    let m = &CACHE.get_or_init(|| SyncMap(Mutex::new(HashMap::new()))).0;
    let mut g = m.lock().unwrap();
    if let Some(&ptr) = g.get(&key) {
        return ptr as *const Lconv;
    }

    let (dec_pt, thou_sep, grp) = if num.is_c {
        (".".to_string(), String::new(), String::new())
    } else if let Some(loc) = kstr::resolve_locale_tag(&num.bcp47) {
        (
            kstr::locale_info_for(&loc, 0x0E).unwrap_or_else(|| ".".into()),
            kstr::locale_info_for(&loc, 0x0F).unwrap_or_else(|| ",".into()),
            grouping_bytes(&kstr::locale_info_for(&loc, 0x10).unwrap_or_else(|| "3;0".into())),
        )
    } else {
        (".".to_string(), ",".to_string(), "\x03".to_string())
    };

    let (
        int_curr,
        curr_sym,
        mon_dec,
        mon_thou,
        mon_grp,
        pos_s,
        neg_s,
        frac_d,
        p_cs_precedes,
        p_sep_by_space,
        n_cs_precedes,
        n_sep_by_space,
        p_sign_posn,
        n_sign_posn,
    ) = if mon.is_c {
        (
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            127u8,
            127u8,
            127u8,
            127u8,
            127u8,
            127u8,
            127u8,
        )
    } else if let Some(loc) = kstr::resolve_locale_tag(&mon.bcp47) {
        let q_u8 = |lctype: u32, def: u8| -> u8 {
            kstr::locale_info_for(&loc, lctype)
                .and_then(|s| s.parse::<u8>().ok())
                .unwrap_or(def)
        };
        (
            kstr::locale_info_for(&loc, 0x15).unwrap_or_else(|| "USD".into()),
            kstr::locale_info_for(&loc, 0x14).unwrap_or_else(|| "$".into()),
            kstr::locale_info_for(&loc, 0x16).unwrap_or_else(|| ".".into()),
            kstr::locale_info_for(&loc, 0x17).unwrap_or_else(|| ",".into()),
            grouping_bytes(&kstr::locale_info_for(&loc, 0x18).unwrap_or_else(|| "3;0".into())),
            kstr::locale_info_for(&loc, 0x50).unwrap_or_default(),
            kstr::locale_info_for(&loc, 0x51).unwrap_or_else(|| "-".into()),
            q_u8(0x19, 2),
            q_u8(0x54, 1),
            q_u8(0x55, 0),
            q_u8(0x56, 1),
            q_u8(0x57, 0),
            q_u8(0x52, 3),
            q_u8(0x53, 1),
        )
    } else {
        (
            "USD".into(),
            "$".into(),
            ".".into(),
            ",".into(),
            "\x03".into(),
            String::new(),
            "-".into(),
            2u8,
            1u8,
            0u8,
            1u8,
            0u8,
            3u8,
            1u8,
        )
    };

    let lc = Box::leak(Box::new(Lconv {
        decimal_point: intern_in_cp(&dec_pt, num.cp),
        thousands_sep: intern_in_cp(&thou_sep, num.cp),
        grouping: intern_ansi(&grp),
        int_curr_symbol: intern_in_cp(&int_curr, mon.cp),
        currency_symbol: intern_in_cp(&curr_sym, mon.cp),
        mon_decimal_point: intern_in_cp(&mon_dec, mon.cp),
        mon_thousands_sep: intern_in_cp(&mon_thou, mon.cp),
        mon_grouping: intern_ansi(&mon_grp),
        positive_sign: intern_in_cp(&pos_s, mon.cp),
        negative_sign: intern_in_cp(&neg_s, mon.cp),
        int_frac_digits: frac_d,
        frac_digits: frac_d,
        p_cs_precedes,
        p_sep_by_space,
        n_cs_precedes,
        n_sep_by_space,
        p_sign_posn,
        n_sign_posn,
        w_decimal_point: intern_wide(&dec_pt),
        w_thousands_sep: intern_wide(&thou_sep),
        w_int_curr_symbol: intern_wide(&int_curr),
        w_currency_symbol: intern_wide(&curr_sym),
        w_mon_decimal_point: intern_wide(&mon_dec),
        w_mon_thousands_sep: intern_wide(&mon_thou),
        w_positive_sign: intern_wide(&pos_s),
        w_negative_sign: intern_wide(&neg_s),
    }));
    let ptr = lc as *const Lconv;
    g.insert(key, ptr as usize);
    ptr
}

pub unsafe extern "win64" fn localeconv() -> *const Lconv {
    let st = global_state().lock().unwrap();
    let num = st.cats[(LC_NUMERIC - 1) as usize].clone();
    let mon = st.cats[(LC_MONETARY - 1) as usize].clone();
    drop(st);
    build_lconv(&num, &mon)
}

pub unsafe extern "win64" fn ___lc_codepage_func() -> u32 {
    CTYPE_CP.load(Ordering::Relaxed)
}
pub unsafe extern "win64" fn ___lc_collate_cp_func() -> u32 {
    COLLATE_CP.load(Ordering::Relaxed)
}
pub unsafe extern "win64" fn ___mb_cur_max_func() -> i32 {
    let cp = CTYPE_CP.load(Ordering::Relaxed);
    if cp == 0 {
        1
    } else {
        kstr::cp_max_char_size(cp) as i32
    }
}
pub unsafe extern "win64" fn ___mb_cur_max_l_func(l: *mut c_void) -> i32 {
    if l.is_null() {
        return ___mb_cur_max_func();
    }
    let obj = &*(l as *const LocaleObj);
    obj.entry.max_char_size()
}

pub unsafe extern "win64" fn ___lc_locale_name_func() -> *const *const u16 {
    let st = global_state().lock().unwrap();
    let mut ptrs = [0usize; 6];
    for i in 0..5 {
        let e = &st.cats[i];
        ptrs[i + 1] = if e.is_c { 0 } else { intern_wide(&e.bcp47) as usize };
    }
    ptrs[0] = ptrs[LC_CTYPE as usize];
    let key = ptrs;
    drop(st);
    struct SyncMap(Mutex<HashMap<[usize; 6], usize>>);
    unsafe impl Sync for SyncMap {}
    static CACHE: OnceLock<SyncMap> = OnceLock::new();
    let m = &CACHE.get_or_init(|| SyncMap(Mutex::new(HashMap::new()))).0;
    let mut g = m.lock().unwrap();
    if let Some(&p) = g.get(&key) {
        return p as *const *const u16;
    }
    let leaked: &'static [usize; 6] = Box::leak(Box::new(key));
    let ptr = leaked.as_ptr() as usize;
    g.insert(key, ptr);
    ptr as *const *const u16
}

static THREAD_LOCALE_MODE: AtomicI32 = AtomicI32::new(2); // _DISABLE_PER_THREAD_LOCALE

pub unsafe extern "win64" fn _configthreadlocale(flag: i32) -> i32 {
    let old = THREAD_LOCALE_MODE.load(Ordering::Relaxed);
    if flag == 1 || flag == 2 {
        THREAD_LOCALE_MODE.store(flag, Ordering::Relaxed);
    } else if flag != 0 && flag != -1 {
        return -1;
    }
    old
}
#[inline(never)]
pub unsafe extern "win64" fn _lock_locales() {}
#[inline(never)]
pub unsafe extern "win64" fn _unlock_locales() {}

/// `_locale_t` handles: heap object carrying the resolved [`LocaleEntry`].
struct LocaleObj {
    entry: LocaleEntry,
}

pub unsafe extern "win64" fn _create_locale(cat: i32, name: *const u8) -> *mut c_void {
    if !(LC_ALL..=LC_TIME).contains(&cat) || name.is_null() {
        return std::ptr::null_mut();
    }
    match parse_single_locale(&from_ansi(name)) {
        Some(entry) => Box::into_raw(Box::new(LocaleObj { entry })) as *mut c_void,
        None => std::ptr::null_mut(),
    }
}
pub unsafe extern "win64" fn _wcreate_locale(cat: i32, name: *const u16) -> *mut c_void {
    if !(LC_ALL..=LC_TIME).contains(&cat) || name.is_null() {
        return std::ptr::null_mut();
    }
    match parse_single_locale(&from_wide(name)) {
        Some(entry) => Box::into_raw(Box::new(LocaleObj { entry })) as *mut c_void,
        None => std::ptr::null_mut(),
    }
}
pub unsafe extern "win64" fn _free_locale(l: *mut c_void) {
    if !l.is_null() {
        drop(Box::from_raw(l as *mut LocaleObj));
    }
}
pub unsafe extern "win64" fn _get_current_locale() -> *mut c_void {
    let entry = global_state().lock().unwrap().cats[(LC_CTYPE - 1) as usize].clone();
    Box::into_raw(Box::new(LocaleObj { entry })) as *mut c_void
}
pub unsafe extern "win64" fn __acrt_get_qualified_locale(_a: *const c_void, _b: *mut c_void, _c: *mut c_void) -> i32 {
    0
}
pub unsafe extern "win64" fn _getmbcp() -> i32 {
    MB_CODEPAGE.load(Ordering::Relaxed)
}
pub unsafe extern "win64" fn _setmbcp(cp: i32) -> i32 {
    let resolved = match cp {
        _MB_CP_SBCS => 0,
        _MB_CP_OEM => 437,
        _MB_CP_ANSI => ACP as i32,
        _MB_CP_LOCALE => mb_cp() as i32,
        c if c > 0 && kstr::valid_cp(c as u32) => kstr::resolve_cp(c as u32) as i32,
        _ => {
            errno::set(EINVAL);
            return -1;
        }
    };
    MB_CODEPAGE.store(resolved, Ordering::Relaxed);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setlocale_real_locales_lconv_and_setmbcp() {
        unsafe {
            // Start in C locale
            let r = setlocale(LC_ALL, b"C\0".as_ptr());
            assert!(!r.is_null());
            assert_eq!(from_ansi(r), "C");
            assert!(c_locale_active());
            assert_eq!(___lc_codepage_func(), 0);
            assert_eq!(___mb_cur_max_func(), 1);

            // Switch to de-DE (German, default CP1252) and inspect real lconv encoded in CP1252!
            let r = setlocale(LC_ALL, b"de-DE\0".as_ptr());
            assert!(!r.is_null());
            assert!(from_ansi(r).contains("German"), "got {}", from_ansi(r));
            assert_eq!(___lc_codepage_func(), 1252);
            let lc = &*localeconv();
            assert_eq!(from_ansi(lc.decimal_point), ",");
            assert_eq!(from_ansi(lc.thousands_sep), ".");
            assert_eq!(cstr_slice(lc.currency_symbol), &[0x80]); // Euro sign in CP1252
            assert_eq!(from_ansi(lc.int_curr_symbol), "EUR");
            assert_eq!(from_wide(lc.w_decimal_point), ",");
            assert_eq!(from_wide(lc.w_currency_symbol), "€");
            // German monetary placement: "1.234,56 €" / "-1.234,56 €"
            assert_eq!(lc.p_cs_precedes, 0);
            assert_eq!(lc.p_sep_by_space, 1);
            assert_eq!(lc.n_cs_precedes, 0);
            assert_eq!(lc.n_sep_by_space, 1);
            assert_eq!(lc.p_sign_posn, 1);
            assert_eq!(lc.n_sign_posn, 1);

            // en-IN secondary grouping propagates into localeconv().grouping as [3, 2].
            assert!(!setlocale(LC_ALL, b"en-IN\0".as_ptr()).is_null());
            let lc_in = &*localeconv();
            assert_eq!(cstr_slice(lc_in.grouping), &[3, 2]);
            assert_eq!(lc_in.p_cs_precedes, 1);
            assert_eq!(lc_in.p_sep_by_space, 0);

            // Restore de-DE before CP1252 mbstowcs/wcstombs assertions.
            assert!(!setlocale(LC_ALL, b"de-DE\0".as_ptr()).is_null());

            // Verify mbstowcs / wcstombs honor CP1252 when LC_CTYPE is de-DE!
            let euro_cp1252 = [0x80u8, 0];
            let mut w_euro = [0u16; 4];
            assert_eq!(super::super::convert::mbstowcs(w_euro.as_mut_ptr(), euro_cp1252.as_ptr(), 4), 1);
            assert_eq!(w_euro[0], 0x20AC);
            let mut m_euro = [0u8; 8];
            assert_eq!(super::super::convert::wcstombs(m_euro.as_mut_ptr(), w_euro.as_ptr(), 8), 1);
            assert_eq!(m_euro[0], 0x80);

            // Switching to de-DE.utf8 encodes narrow currency_symbol in UTF-8 without reusing CP1252 cache!
            let r = setlocale(LC_ALL, b"de-DE.utf8\0".as_ptr());
            assert!(!r.is_null());
            assert_eq!(___lc_codepage_func(), 65001);
            let lc_u8 = &*localeconv();
            assert_eq!(cstr_slice(lc_u8.currency_symbol), "€".as_bytes());
            assert_eq!(from_wide(lc_u8.w_currency_symbol), "€");

            // Legacy CRT name with Shift-JIS codepage (932 -> MB_CUR_MAX == 2)
            let r = setlocale(LC_ALL, b"Japanese_Japan.932\0".as_ptr());
            assert!(!r.is_null());
            assert_eq!(___lc_codepage_func(), 932);
            assert_eq!(___mb_cur_max_func(), 2);

            // Per-category setlocale + composite LC_ALL query & roundtrip
            assert!(!setlocale(LC_NUMERIC, b"C\0".as_ptr()).is_null());
            let comp = from_ansi(setlocale(LC_ALL, std::ptr::null()));
            assert!(comp.contains("LC_NUMERIC=C"), "composite={comp}");
            assert_eq!(from_ansi((*localeconv()).decimal_point), ".");

            // Invalid locale returns NULL and preserves state
            assert!(setlocale(LC_ALL, b"not_a_real_locale_xyz\0".as_ptr()).is_null());

            // _create_locale + ___mb_cur_max_l_func
            let l_utf8 = _create_locale(LC_ALL, b"fr-FR.utf8\0".as_ptr());
            assert!(!l_utf8.is_null());
            assert_eq!(___mb_cur_max_l_func(l_utf8), 4);
            _free_locale(l_utf8);

            // _setmbcp / _getmbcp across real Windows code pages and special constants
            assert_eq!(_setmbcp(932), 0);
            assert_eq!(_getmbcp(), 932);
            assert_eq!(_setmbcp(1252), 0);
            assert_eq!(_getmbcp(), 1252);
            assert_eq!(_setmbcp(_MB_CP_SBCS), 0);
            assert_eq!(_getmbcp(), 0);
            assert_eq!(_setmbcp(_MB_CP_ANSI), 0);
            assert_eq!(_getmbcp(), ACP as i32);
            assert_eq!(_setmbcp(999999), -1);
            assert_eq!(errno::get(), EINVAL);
            assert_eq!(_getmbcp(), ACP as i32);

            // Restore default C locale for subsequent serial tests
            assert!(!setlocale(LC_ALL, b"C\0".as_ptr()).is_null());
        }
    }
}
