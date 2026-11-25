//! Window classes: `RegisterClass[Ex][A|W]`, `UnregisterClass`, `GetClassInfo`.
//!
//! One process-wide table. Classes are looked up by atom or by
//! case-insensitive name (the `hInstance` part of the key is ignored, which
//! only matters for apps registering the same name from two modules).

use super::*;
use crate::types::*;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

pub const CS_VREDRAW: u32 = 0x0001;
pub const CS_HREDRAW: u32 = 0x0002;
pub const CS_DBLCLKS: u32 = 0x0008;
pub const CS_PARENTDC: u32 = 0x0080;
pub const CS_SAVEBITS: u32 = 0x0800;
pub const CS_GLOBALCLASS: u32 = 0x4000;

/// Predefined dialog box class atom (`WC_DIALOG` = `#32770`).
pub const ATOM_DIALOG: ATOM = 32770;
const DLGWINDOWEXTRA: i32 = 30;

#[derive(Debug)]
pub struct Class {
    pub atom: ATOM,
    pub name: String,
    pub style: AtomicU32,
    pub wndproc: AtomicUsize,
    pub cls_extra: i32,
    pub wnd_extra: i32,
    pub instance: AtomicUsize,
    pub icon: AtomicUsize,
    pub icon_sm: AtomicUsize,
    pub cursor: Mutex<usize>,
    pub background: AtomicUsize,
    pub menu_name: AtomicUsize,
    pub extra: Mutex<Vec<u8>>,
    pub unicode: bool,
    /// Built-in USER32 system global class (cannot be unregistered by applications).
    pub system: bool,
}

struct Table {
    next_atom: ATOM,
    classes: Vec<Arc<Class>>,
}

fn builtin_system_classes(next_atom: &mut ATOM) -> Vec<Arc<Class>> {
    let mut classes = Vec::new();
    let mut push_sys = |atom_override: Option<ATOM>, name: &str, style: u32, wndproc: usize, wnd_extra: i32| {
        let atom = match atom_override {
            Some(a) => a,
            None => {
                let a = *next_atom;
                *next_atom = next_atom.wrapping_add(1);
                a
            }
        };
        classes.push(Arc::new(Class {
            atom,
            name: name.to_string(),
            style: AtomicU32::new(style | CS_GLOBALCLASS),
            wndproc: AtomicUsize::new(wndproc),
            cls_extra: 0,
            wnd_extra,
            instance: AtomicUsize::new(0),
            icon: AtomicUsize::new(0),
            icon_sm: AtomicUsize::new(0),
            cursor: Mutex::new(0),
            background: AtomicUsize::new(0),
            menu_name: AtomicUsize::new(0),
            extra: Mutex::new(Vec::new()),
            unicode: true,
            system: true,
        }));
    };

    push_sys(
        Some(ATOM_DIALOG),
        "#32770",
        CS_DBLCLKS | CS_SAVEBITS,
        super::WNDPROC_HANDLE_DEF_DLG_W,
        DLGWINDOWEXTRA,
    );
    push_sys(
        None,
        "Button",
        CS_DBLCLKS | CS_HREDRAW | CS_VREDRAW | CS_PARENTDC,
        super::WNDPROC_HANDLE_BUTTON,
        0,
    );
    push_sys(
        None,
        "Edit",
        CS_DBLCLKS | CS_PARENTDC,
        super::WNDPROC_HANDLE_EDIT,
        0,
    );
    push_sys(
        None,
        "Static",
        CS_DBLCLKS | CS_PARENTDC,
        super::WNDPROC_HANDLE_STATIC,
        0,
    );
    push_sys(
        None,
        "ListBox",
        CS_DBLCLKS | CS_PARENTDC,
        super::WNDPROC_HANDLE_DEFAULT_CTRL,
        0,
    );
    push_sys(
        None,
        "ScrollBar",
        CS_DBLCLKS | CS_HREDRAW | CS_VREDRAW | CS_PARENTDC,
        super::WNDPROC_HANDLE_DEFAULT_CTRL,
        0,
    );
    push_sys(
        None,
        "ComboBox",
        CS_DBLCLKS | CS_HREDRAW | CS_VREDRAW | CS_PARENTDC,
        super::WNDPROC_HANDLE_DEFAULT_CTRL,
        0,
    );
    classes
}

fn table() -> &'static Mutex<Table> {
    static T: OnceLock<Mutex<Table>> = OnceLock::new();
    T.get_or_init(|| {
        let mut next_atom = 0xC000;
        let classes = builtin_system_classes(&mut next_atom);
        Mutex::new(Table { next_atom, classes })
    })
}

/// Class name argument: an atom (`MAKEINTATOM`) or a string.
pub enum ClassName {
    Atom(ATOM),
    Name(String),
}

impl ClassName {
    pub unsafe fn from_w(p: LPCWSTR) -> ClassName {
        if is_int_resource(p) {
            ClassName::Atom(p as usize as ATOM)
        } else {
            ClassName::Name(crate::strings::from_wide(p))
        }
    }
    pub unsafe fn from_a(p: LPCSTR) -> ClassName {
        if is_int_resource(p as *const u16) {
            ClassName::Atom(p as usize as ATOM)
        } else {
            ClassName::Name(crate::strings::from_ansi(p))
        }
    }
}

impl std::fmt::Display for ClassName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassName::Atom(a) => write!(f, "#{a:#x}"),
            ClassName::Name(n) => write!(f, "{n:?}"),
        }
    }
}

pub fn find(name: &ClassName) -> Option<Arc<Class>> {
    let t = table().lock().unwrap();
    t.classes
        .iter()
        .find(|c| match name {
            ClassName::Atom(a) => c.atom == *a,
            ClassName::Name(n) => c.name.eq_ignore_ascii_case(n),
        })
        .cloned()
}

pub struct ClassDesc {
    pub name: String,
    pub style: u32,
    pub wndproc: usize,
    pub cls_extra: i32,
    pub wnd_extra: i32,
    pub instance: usize,
    pub icon: usize,
    pub cursor: usize,
    pub background: usize,
    pub unicode: bool,
}

pub fn register(d: ClassDesc) -> ATOM {
    if d.wndproc == 0 || d.cls_extra < 0 || d.wnd_extra < 0 {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    let mut t = table().lock().unwrap();
    if t.classes.iter().any(|c| c.name.eq_ignore_ascii_case(&d.name)) {
        set_error(e::CLASS_ALREADY_EXISTS);
        return 0;
    }
    let atom = t.next_atom;
    t.next_atom += 1;
    log::debug!("RegisterClass {:?} atom {atom:#x} style {:#x} wndproc {:#x}", d.name, d.style, d.wndproc);
    t.classes.push(Arc::new(Class {
        atom,
        name: d.name,
        style: AtomicU32::new(d.style),
        wndproc: AtomicUsize::new(d.wndproc),
        cls_extra: d.cls_extra,
        wnd_extra: d.wnd_extra,
        instance: AtomicUsize::new(d.instance),
        icon: AtomicUsize::new(d.icon),
        icon_sm: AtomicUsize::new(d.icon),
        cursor: Mutex::new(d.cursor),
        background: AtomicUsize::new(d.background),
        menu_name: AtomicUsize::new(0),
        extra: Mutex::new(vec![0u8; d.cls_extra.max(0) as usize]),
        unicode: d.unicode,
        system: false,
    }));
    atom
}

/// `WNDCLASSW` / `WNDCLASSA` (identical layout; string pointer types differ).
#[repr(C)]
pub struct WNDCLASS {
    pub style: u32,
    pub wndproc: usize,
    pub cls_extra: i32,
    pub wnd_extra: i32,
    pub instance: usize,
    pub icon: usize,
    pub cursor: usize,
    pub background: usize,
    pub menu_name: usize,
    pub class_name: usize,
}
const _: () = assert!(std::mem::size_of::<WNDCLASS>() == 72);

/// `WNDCLASSEXW` / `WNDCLASSEXA`.
#[repr(C)]
pub struct WNDCLASSEX {
    pub cb_size: u32,
    pub style: u32,
    pub wndproc: usize,
    pub cls_extra: i32,
    pub wnd_extra: i32,
    pub instance: usize,
    pub icon: usize,
    pub cursor: usize,
    pub background: usize,
    pub menu_name: usize,
    pub class_name: usize,
    pub icon_sm: usize,
}
const _: () = assert!(std::mem::size_of::<WNDCLASSEX>() == 80);

unsafe fn class_name_string(p: usize, unicode: bool) -> Option<String> {
    if p == 0 {
        return None;
    }
    if is_int_resource(p as *const u16) {
        // Registering by atom name is legal but unusual; give it a stable name.
        return Some(format!("#{}", p));
    }
    Some(if unicode { crate::strings::from_wide(p as *const u16) } else { crate::strings::from_ansi(p as *const u8) })
}

unsafe fn register_common(c: &WNDCLASS, unicode: bool) -> ATOM {
    let Some(name) = class_name_string(c.class_name, unicode) else {
        set_error(e::INVALID_PARAMETER);
        return 0;
    };
    register(ClassDesc {
        name,
        style: c.style,
        wndproc: c.wndproc,
        cls_extra: c.cls_extra,
        wnd_extra: c.wnd_extra,
        instance: c.instance,
        icon: c.icon,
        cursor: c.cursor,
        background: c.background,
        unicode,
    })
}

unsafe fn ex_to_plain(c: &WNDCLASSEX) -> WNDCLASS {
    WNDCLASS {
        style: c.style,
        wndproc: c.wndproc,
        cls_extra: c.cls_extra,
        wnd_extra: c.wnd_extra,
        instance: c.instance,
        icon: c.icon,
        cursor: c.cursor,
        background: c.background,
        menu_name: c.menu_name,
        class_name: c.class_name,
    }
}

pub unsafe extern "win64" fn RegisterClassW(c: *const WNDCLASS) -> ATOM {
    if c.is_null() {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    register_common(&*c, true)
}

pub unsafe extern "win64" fn RegisterClassA(c: *const WNDCLASS) -> ATOM {
    if c.is_null() {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    register_common(&*c, false)
}

pub unsafe extern "win64" fn RegisterClassExW(c: *const WNDCLASSEX) -> ATOM {
    if c.is_null() || (*c).cb_size as usize != std::mem::size_of::<WNDCLASSEX>() {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    register_common(&ex_to_plain(&*c), true)
}

pub unsafe extern "win64" fn RegisterClassExA(c: *const WNDCLASSEX) -> ATOM {
    if c.is_null() || (*c).cb_size as usize != std::mem::size_of::<WNDCLASSEX>() {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    register_common(&ex_to_plain(&*c), false)
}

fn unregister(name: ClassName) -> BOOL {
    let Some(c) = find(&name) else {
        set_error(e::CLASS_DOES_NOT_EXIST);
        return FALSE;
    };
    if c.system {
        set_error(e::CLASS_DOES_NOT_EXIST);
        return FALSE;
    }
    if super::window::any_window_of_class(c.atom) {
        set_error(e::CLASS_HAS_WINDOWS);
        return FALSE;
    }
    table().lock().unwrap().classes.retain(|x| x.atom != c.atom);
    TRUE
}

pub unsafe extern "win64" fn UnregisterClassW(name: LPCWSTR, _inst: HINSTANCE) -> BOOL {
    unregister(ClassName::from_w(name))
}

pub unsafe extern "win64" fn UnregisterClassA(name: LPCSTR, _inst: HINSTANCE) -> BOOL {
    unregister(ClassName::from_a(name))
}

unsafe fn fill_info(c: &Class, out: *mut WNDCLASSEX, ex: bool) {
    if ex {
        let o = &mut *out;
        o.style = c.style.load(Ordering::Relaxed);
        o.wndproc = c.wndproc.load(Ordering::Relaxed);
        o.cls_extra = c.cls_extra;
        o.wnd_extra = c.wnd_extra;
        o.instance = c.instance.load(Ordering::Relaxed);
        o.icon = c.icon.load(Ordering::Relaxed);
        o.cursor = *c.cursor.lock().unwrap();
        o.background = c.background.load(Ordering::Relaxed);
        o.menu_name = c.menu_name.load(Ordering::Relaxed);
        o.icon_sm = c.icon_sm.load(Ordering::Relaxed);
        // class_name is left as passed in by the caller (documented behaviour).
    } else {
        let o = &mut *(out as *mut WNDCLASS);
        o.style = c.style.load(Ordering::Relaxed);
        o.wndproc = c.wndproc.load(Ordering::Relaxed);
        o.cls_extra = c.cls_extra;
        o.wnd_extra = c.wnd_extra;
        o.instance = c.instance.load(Ordering::Relaxed);
        o.icon = c.icon.load(Ordering::Relaxed);
        o.cursor = *c.cursor.lock().unwrap();
        o.background = c.background.load(Ordering::Relaxed);
        o.menu_name = c.menu_name.load(Ordering::Relaxed);
    }
}

unsafe fn get_class_info(name: ClassName, out: *mut WNDCLASSEX, ex: bool) -> BOOL {
    if out.is_null() {
        set_error(e::INVALID_PARAMETER);
        return FALSE;
    }
    match find(&name) {
        Some(c) => {
            fill_info(&c, out, ex);
            c.atom as BOOL
        }
        None => {
            set_error(e::CLASS_DOES_NOT_EXIST);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn GetClassInfoExW(_inst: HINSTANCE, name: LPCWSTR, out: *mut WNDCLASSEX) -> BOOL {
    get_class_info(ClassName::from_w(name), out, true)
}

pub unsafe extern "win64" fn GetClassInfoW(_inst: HINSTANCE, name: LPCWSTR, out: *mut WNDCLASS) -> BOOL {
    get_class_info(ClassName::from_w(name), out as *mut WNDCLASSEX, false)
}

pub unsafe extern "win64" fn GetClassInfoExA(_inst: HINSTANCE, name: LPCSTR, out: *mut WNDCLASSEX) -> BOOL {
    get_class_info(ClassName::from_a(name), out, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe extern "win64" fn proc_(_h: usize, _m: u32, _w: WPARAM, _l: LPARAM) -> LRESULT {
        0
    }

    #[test]
    fn register_find_unregister() {
        let name = crate::strings::to_wide_z("Aw2TestClassA");
        let wc = WNDCLASS {
            style: CS_DBLCLKS,
            wndproc: proc_ as *const () as usize,
            cls_extra: 0,
            wnd_extra: 8,
            instance: 0,
            icon: 0,
            cursor: 0,
            background: 0,
            menu_name: 0,
            class_name: name.as_ptr() as usize,
        };
        unsafe {
            let a = RegisterClassW(&wc);
            assert!(a >= 0xC000);
            assert_eq!(RegisterClassW(&wc), 0, "duplicate registration must fail");
            assert_eq!(find(&ClassName::Atom(a)).unwrap().wnd_extra, 8);
            assert!(find(&ClassName::Name("aw2testclassa".into())).is_some());
            assert_eq!(UnregisterClassW(name.as_ptr(), std::ptr::null_mut()), TRUE);
            assert!(find(&ClassName::Atom(a)).is_none());
        }
    }

    #[test]
    fn nonclient_metrics_a_and_w_succeed() {
        use super::misc::{LogFontA, LogFontW, NonClientMetricsA, NonClientMetricsW};
        use core::mem::{offset_of, size_of};

        // Verify exact documented Win32 struct sizes and field byte offsets.
        assert_eq!(size_of::<LogFontA>(), 60);
        assert_eq!(size_of::<LogFontW>(), 92);
        assert_eq!(size_of::<NonClientMetricsA>(), 344);
        assert_eq!(size_of::<NonClientMetricsW>(), 504);

        assert_eq!(offset_of!(NonClientMetricsA, cb_size), 0);
        assert_eq!(offset_of!(NonClientMetricsA, i_border_width), 4);
        assert_eq!(offset_of!(NonClientMetricsA, i_scroll_width), 8);
        assert_eq!(offset_of!(NonClientMetricsA, i_scroll_height), 12);
        assert_eq!(offset_of!(NonClientMetricsA, i_caption_width), 16);
        assert_eq!(offset_of!(NonClientMetricsA, i_caption_height), 20);
        assert_eq!(offset_of!(NonClientMetricsA, lf_caption_font), 24);
        assert_eq!(offset_of!(NonClientMetricsA, i_sm_caption_width), 84);
        assert_eq!(offset_of!(NonClientMetricsA, i_sm_caption_height), 88);
        assert_eq!(offset_of!(NonClientMetricsA, lf_sm_caption_font), 92);
        assert_eq!(offset_of!(NonClientMetricsA, i_menu_width), 152);
        assert_eq!(offset_of!(NonClientMetricsA, i_menu_height), 156);
        assert_eq!(offset_of!(NonClientMetricsA, lf_menu_font), 160);
        assert_eq!(offset_of!(NonClientMetricsA, lf_status_font), 220);
        assert_eq!(offset_of!(NonClientMetricsA, lf_message_font), 280);
        assert_eq!(offset_of!(NonClientMetricsA, i_padded_border_width), 340);

        assert_eq!(offset_of!(NonClientMetricsW, cb_size), 0);
        assert_eq!(offset_of!(NonClientMetricsW, i_border_width), 4);
        assert_eq!(offset_of!(NonClientMetricsW, i_scroll_width), 8);
        assert_eq!(offset_of!(NonClientMetricsW, i_scroll_height), 12);
        assert_eq!(offset_of!(NonClientMetricsW, i_caption_width), 16);
        assert_eq!(offset_of!(NonClientMetricsW, i_caption_height), 20);
        assert_eq!(offset_of!(NonClientMetricsW, lf_caption_font), 24);
        assert_eq!(offset_of!(NonClientMetricsW, i_sm_caption_width), 116);
        assert_eq!(offset_of!(NonClientMetricsW, i_sm_caption_height), 120);
        assert_eq!(offset_of!(NonClientMetricsW, lf_sm_caption_font), 124);
        assert_eq!(offset_of!(NonClientMetricsW, i_menu_width), 216);
        assert_eq!(offset_of!(NonClientMetricsW, i_menu_height), 220);
        assert_eq!(offset_of!(NonClientMetricsW, lf_menu_font), 224);
        assert_eq!(offset_of!(NonClientMetricsW, lf_status_font), 316);
        assert_eq!(offset_of!(NonClientMetricsW, lf_message_font), 408);
        assert_eq!(offset_of!(NonClientMetricsW, i_padded_border_width), 500);

        unsafe {
            // NONCLIENTMETRICSA (Vista+ 344 bytes)
            let mut ncm_a = vec![0u8; 344];
            (ncm_a.as_mut_ptr() as *mut u32).write_unaligned(344);
            assert_eq!(
                super::misc::SystemParametersInfoA(0x0029, 344, ncm_a.as_mut_ptr(), 0),
                TRUE
            );
            assert_eq!((ncm_a.as_ptr().add(4) as *const i32).read_unaligned(), 1);
            assert_eq!((ncm_a.as_ptr().add(8) as *const i32).read_unaligned(), 17);
            assert_eq!((ncm_a.as_ptr().add(16) as *const i32).read_unaligned(), 36);
            assert_eq!((ncm_a.as_ptr().add(24) as *const i32).read_unaligned(), -12);
            assert_eq!(&ncm_a[24 + 28..24 + 28 + 9], b"Segoe UI\0");
            assert_eq!((ncm_a.as_ptr().add(84) as *const i32).read_unaligned(), 22);
            assert_eq!((ncm_a.as_ptr().add(88) as *const i32).read_unaligned(), 22);
            assert_eq!(&ncm_a[92 + 28..92 + 28 + 9], b"Segoe UI\0");
            assert_eq!((ncm_a.as_ptr().add(152) as *const i32).read_unaligned(), 19);
            assert_eq!((ncm_a.as_ptr().add(156) as *const i32).read_unaligned(), 19);
            assert_eq!(&ncm_a[160 + 28..160 + 28 + 9], b"Segoe UI\0");
            assert_eq!(&ncm_a[220 + 28..220 + 28 + 9], b"Segoe UI\0");
            assert_eq!(&ncm_a[280 + 28..280 + 28 + 9], b"Segoe UI\0");
            assert_eq!((ncm_a.as_ptr().add(340) as *const i32).read_unaligned(), 4);

            // Pre-Vista 340-byte NONCLIENTMETRICSA must not overwrite past byte 340.
            let mut ncm_a_old = vec![0xAAu8; 344];
            (ncm_a_old.as_mut_ptr() as *mut u32).write_unaligned(340);
            assert_eq!(
                super::misc::SystemParametersInfoA(0x0029, 340, ncm_a_old.as_mut_ptr(), 0),
                TRUE
            );
            assert_eq!(&ncm_a_old[340..344], &[0xAAu8; 4]);

            // NONCLIENTMETRICSW (Vista+ 504 bytes)
            let mut ncm_w = vec![0u8; 504];
            (ncm_w.as_mut_ptr() as *mut u32).write_unaligned(504);
            assert_eq!(
                super::misc::SystemParametersInfoW(0x0029, 504, ncm_w.as_mut_ptr(), 0),
                TRUE
            );
            assert_eq!((ncm_w.as_ptr().add(24) as *const i32).read_unaligned(), -12);
            assert_eq!(
                crate::strings::from_wide(ncm_w.as_ptr().add(24 + 28) as *const u16),
                "Segoe UI"
            );
            assert_eq!((ncm_w.as_ptr().add(116) as *const i32).read_unaligned(), 22);
            assert_eq!((ncm_w.as_ptr().add(120) as *const i32).read_unaligned(), 22);
            assert_eq!(
                crate::strings::from_wide(ncm_w.as_ptr().add(124 + 28) as *const u16),
                "Segoe UI"
            );
            assert_eq!((ncm_w.as_ptr().add(216) as *const i32).read_unaligned(), 19);
            assert_eq!((ncm_w.as_ptr().add(220) as *const i32).read_unaligned(), 19);
            assert_eq!(
                crate::strings::from_wide(ncm_w.as_ptr().add(224 + 28) as *const u16),
                "Segoe UI"
            );
            assert_eq!(
                crate::strings::from_wide(ncm_w.as_ptr().add(316 + 28) as *const u16),
                "Segoe UI"
            );
            assert_eq!(
                crate::strings::from_wide(ncm_w.as_ptr().add(408 + 28) as *const u16),
                "Segoe UI"
            );
            assert_eq!((ncm_w.as_ptr().add(500) as *const i32).read_unaligned(), 4);
        }
    }
}
