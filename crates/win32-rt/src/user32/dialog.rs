//! Win32 dialog boxes: `DLGTEMPLATE`/`DLGTEMPLATEEX` parsing, modeless and
//! modal dialog creation (`CreateDialog*`, `DialogBox*`, `EndDialog`),
//! `DefDlgProcW/A`, standard built-in dialog/control classes, and dialog item
//! helpers (`GetDlgItem`, `SetDlgItemText*`, `GetDlgItemText*`, `SetDlgItemInt`,
//! `GetDlgItemInt`, `SendDlgItemMessage*`, `IsDialogMessage*`,
//! `CheckDlgButton`, `IsDlgButtonChecked`, `GetNextDlgTabItem`).

use super::class::{self, ClassDesc, ClassName};
use super::defproc::DefWindowProcW;
use super::input;
use super::message::{self, send_message, DispatchMessageA, DispatchMessageW, GetMessageW, PostQuitMessage, SendMessageA, SendMessageW, TranslateMessage};
use super::window::{self, CreateWindowExW, DestroyWindow, EnableWindow, GetWindowLongPtrW, GetWindowTextA, GetWindowTextW, SetWindowLongPtrW, SetWindowTextA, SetWindowTextW, ShowWindow, SW_HIDE, SW_SHOWNORMAL};
use super::*;
use crate::types::*;
use std::collections::HashMap;
use std::sync::{Mutex, Once, OnceLock};

pub const DWLP_MSGRESULT: i32 = 0;
pub const DWLP_DLGPROC: i32 = 8;
pub const DWLP_USER: i32 = 16;

const DLGWINDOWEXTRA: i32 = 30;

const RT_DIALOG: usize = 5;

#[allow(dead_code)]
const DS_FIXEDSYS: u32 = 0x0000_0008;
const DS_SETFONT: u32 = 0x0000_0040;
#[allow(dead_code)]
const DS_SHELLFONT: u32 = DS_SETFONT | DS_FIXEDSYS;

const BS_TYPEMASK: u32 = 0x0000_000F;
const BS_DEFPUSHBUTTON: u32 = 0x0000_0001;

const BN_CLICKED: usize = 0;
const IDOK: i32 = 1;
const IDCANCEL: i32 = 2;

const DM_GETDEFID: u32 = wm::USER;
const DM_SETDEFID: u32 = wm::USER + 1;
const DC_HASDEFID: usize = 0x534B;

const DLGC_WANTARROWS: isize = 0x0001;
const DLGC_WANTTAB: isize = 0x0002;
const DLGC_WANTALLKEYS: isize = 0x0004;
const DLGC_WANTMESSAGE: isize = 0x0004;
const DLGC_WANTCHARS: isize = 0x0080;
const DLGC_BUTTON: isize = 0x2000;
const DLGC_DEFPUSHBUTTON: isize = 0x0010;
const DLGC_UNDEFPUSHBUTTON: isize = 0x0020;

const VK_TAB: usize = 0x09;
const VK_RETURN: usize = 0x0D;
const VK_ESCAPE: usize = 0x1B;

const MAX_DIALOG_ITEMS: usize = 4096;
const MAX_TEMPLATE_STR_LEN: usize = 4096;

#[derive(Debug)]
struct DialogState {
    modal: bool,
    ended: bool,
    result: isize,
    def_id: u16,
}

fn dialogs() -> &'static Mutex<HashMap<usize, DialogState>> {
    static D: OnceLock<Mutex<HashMap<usize, DialogState>>> = OnceLock::new();
    D.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Per-button check state (`BST_UNCHECKED = 0`, `BST_CHECKED = 1`, `BST_INDETERMINATE = 2`).
fn button_checks() -> &'static Mutex<HashMap<usize, u32>> {
    static B: OnceLock<Mutex<HashMap<usize, u32>>> = OnceLock::new();
    B.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Clean up dialog and control metadata when an HWND is freed.
pub(crate) fn forget_dialog(hwnd: usize) {
    dialogs().lock().unwrap_or_else(|e| e.into_inner()).remove(&hwnd);
    button_checks().lock().unwrap_or_else(|e| e.into_inner()).remove(&hwnd);
}

// ---------------------------------------------------------------------------
// Built-in system dialog & control window classes
// ---------------------------------------------------------------------------

pub(crate) unsafe extern "win64" fn button_wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let h = hwnd as usize;
    match msg {
        wm::BM_GETCHECK => button_checks().lock().unwrap_or_else(|e| e.into_inner()).get(&h).copied().unwrap_or(0) as LRESULT,
        wm::BM_SETCHECK => {
            button_checks().lock().unwrap_or_else(|e| e.into_inner()).insert(h, wparam as u32);
            0
        }
        wm::BM_CLICK => {
            if let Some(w) = window::get(h) {
                let (parent, id) = {
                    let d = w.data();
                    (d.parent, d.id as u16)
                };
                if parent != 0 {
                    send_message(parent, wm::COMMAND, ((BN_CLICKED & 0xFFFF) << 16) | (id as usize), h as LPARAM);
                }
            }
            0
        }
        wm::GETDLGCODE => {
            let style = window::get(h).map(|w| w.data().style).unwrap_or(0);
            if (style & BS_TYPEMASK) == BS_DEFPUSHBUTTON {
                DLGC_BUTTON | DLGC_DEFPUSHBUTTON
            } else {
                DLGC_BUTTON | DLGC_UNDEFPUSHBUTTON
            }
        }
        wm::NCDESTROY => {
            button_checks().lock().unwrap_or_else(|e| e.into_inner()).remove(&h);
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

pub(crate) unsafe extern "win64" fn edit_wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        wm::GETDLGCODE => DLGC_WANTARROWS | DLGC_WANTCHARS,
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

pub(crate) unsafe extern "win64" fn default_control_wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    DefWindowProcW(hwnd, msg, wparam, lparam)
}

fn ensure_dialog_classes() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let reg = |name: &str, wndproc: usize, wnd_extra: i32| {
            if class::find(&ClassName::Name(name.to_string())).is_none() {
                let _ = class::register(ClassDesc {
                    name: name.to_string(),
                    style: 0,
                    wndproc,
                    cls_extra: 0,
                    wnd_extra,
                    instance: 0,
                    icon: 0,
                    cursor: 0,
                    background: 0,
                    unicode: true,
                });
            }
        };

        reg("#32770", super::WNDPROC_HANDLE_DEF_DLG_W, DLGWINDOWEXTRA);
        reg("Button", super::WNDPROC_HANDLE_BUTTON, 0);
        reg("Edit", super::WNDPROC_HANDLE_EDIT, 0);
        reg("Static", super::WNDPROC_HANDLE_DEFAULT_CTRL, 0);
        reg("ListBox", super::WNDPROC_HANDLE_DEFAULT_CTRL, 0);
        reg("ScrollBar", super::WNDPROC_HANDLE_DEFAULT_CTRL, 0);
        reg("ComboBox", super::WNDPROC_HANDLE_DEFAULT_CTRL, 0);
    });
}

// ---------------------------------------------------------------------------
// DLGTEMPLATE / DLGTEMPLATEEX binary parser (unaligned-safe)
// ---------------------------------------------------------------------------

#[derive(Debug)]
enum TemplateClass {
    DefaultDialog,
    Atom(u16),
    Name(Vec<u16>),
}

#[derive(Debug)]
struct ParsedItem {
    style: u32,
    ex_style: u32,
    x: i16,
    y: i16,
    cx: i16,
    cy: i16,
    id: u32,
    class: TemplateClass,
    title: Vec<u16>,
    extra_ptr: usize,
}

#[derive(Debug)]
struct ParsedTemplate {
    style: u32,
    ex_style: u32,
    x: i16,
    y: i16,
    cx: i16,
    cy: i16,
    class: TemplateClass,
    title: Vec<u16>,
    items: Vec<ParsedItem>,
}

struct TemplateCursor {
    base: *const u8,
    off: usize,
    limit: Option<usize>,
}

impl TemplateCursor {
    fn new(base: *const u8, limit: Option<usize>) -> Self {
        Self { base, off: 0, limit }
    }

    fn check(&self, need: usize) -> Result<(), ()> {
        let end = self.off.checked_add(need).ok_or(())?;
        if let Some(lim) = self.limit {
            if end > lim {
                return Err(());
            }
        }
        Ok(())
    }

    unsafe fn read_u8(&mut self) -> Result<u8, ()> {
        self.check(1)?;
        let v = self.base.add(self.off).read_unaligned();
        self.off += 1;
        Ok(v)
    }

    unsafe fn read_u16(&mut self) -> Result<u16, ()> {
        self.check(2)?;
        let v = (self.base.add(self.off) as *const u16).read_unaligned();
        self.off += 2;
        Ok(v)
    }

    unsafe fn read_i16(&mut self) -> Result<i16, ()> {
        Ok(self.read_u16()? as i16)
    }

    unsafe fn read_u32(&mut self) -> Result<u32, ()> {
        self.check(4)?;
        let v = (self.base.add(self.off) as *const u32).read_unaligned();
        self.off += 4;
        Ok(v)
    }

    fn align_dword(&mut self) -> Result<(), ()> {
        let aligned = (self.off + 3) & !3;
        if let Some(lim) = self.limit {
            if aligned > lim {
                return Err(());
            }
        }
        self.off = aligned;
        Ok(())
    }

    unsafe fn read_wide_z(&mut self) -> Result<Vec<u16>, ()> {
        let mut out = Vec::new();
        for _ in 0..MAX_TEMPLATE_STR_LEN {
            let u = self.read_u16()?;
            if u == 0 {
                return Ok(out);
            }
            out.push(u);
        }
        Err(())
    }

    unsafe fn read_sz_or_ord(&mut self) -> Result<(Option<u16>, Vec<u16>), ()> {
        let first = self.read_u16()?;
        match first {
            0x0000 => Ok((None, Vec::new())),
            0xFFFF => {
                let ord = self.read_u16()?;
                Ok((Some(ord), Vec::new()))
            }
            c => {
                let mut out = vec![c];
                for _ in 1..MAX_TEMPLATE_STR_LEN {
                    let u = self.read_u16()?;
                    if u == 0 {
                        return Ok((None, out));
                    }
                    out.push(u);
                }
                Err(())
            }
        }
    }
}

fn predefined_control_class_name(ord: u16) -> Option<&'static str> {
    match ord {
        0x0080 => Some("Button"),
        0x0081 => Some("Edit"),
        0x0082 => Some("Static"),
        0x0083 => Some("ListBox"),
        0x0084 => Some("ScrollBar"),
        0x0085 => Some("ComboBox"),
        _ => None,
    }
}

unsafe fn parse_dialog_template(ptr: *const u8, limit: Option<usize>) -> Result<ParsedTemplate, ()> {
    if ptr.is_null() {
        return Err(());
    }
    let mut cur = TemplateCursor::new(ptr, limit);
    cur.check(4)?;
    let w0 = (ptr as *const u16).read_unaligned();
    let w1 = (ptr.add(2) as *const u16).read_unaligned();

    let is_ex = if w1 == 0xFFFF {
        if w0 != 1 {
            return Err(());
        }
        true
    } else {
        false
    };

    let (style, ex_style, cdit, x, y, cx, cy) = if is_ex {
        let _dlg_ver = cur.read_u16()?;
        let _sig = cur.read_u16()?;
        let _help_id = cur.read_u32()?;
        let ex_style = cur.read_u32()?;
        let style = cur.read_u32()?;
        let cdit = cur.read_u16()?;
        let x = cur.read_i16()?;
        let y = cur.read_i16()?;
        let cx = cur.read_i16()?;
        let cy = cur.read_i16()?;
        (style, ex_style, cdit, x, y, cx, cy)
    } else {
        let style = cur.read_u32()?;
        let ex_style = cur.read_u32()?;
        let cdit = cur.read_u16()?;
        let x = cur.read_i16()?;
        let y = cur.read_i16()?;
        let cx = cur.read_i16()?;
        let cy = cur.read_i16()?;
        (style, ex_style, cdit, x, y, cx, cy)
    };

    if cdit as usize > MAX_DIALOG_ITEMS {
        return Err(());
    }

    // Menu (sz_Or_Ord)
    let _ = cur.read_sz_or_ord()?;

    // Dialog class (sz_Or_Ord)
    let (cls_ord, cls_name) = cur.read_sz_or_ord()?;
    let class = match (cls_ord, cls_name.is_empty()) {
        (Some(ord), _) => TemplateClass::Atom(ord),
        (None, true) => TemplateClass::DefaultDialog,
        (None, false) => TemplateClass::Name(cls_name),
    };

    // Title
    let title = cur.read_wide_z()?;

    // Optional font info is present iff DS_SETFONT is set (DS_SHELLFONT = DS_SETFONT | DS_FIXEDSYS;
    // DS_FIXEDSYS alone selects the fixed system font with no font block in the template).
    if (style & DS_SETFONT) != 0 {
        let _point_size = cur.read_u16()?;
        if is_ex {
            let _weight = cur.read_u16()?;
            let _italic = cur.read_u8()?;
            let _charset = cur.read_u8()?;
        }
        let _typeface = cur.read_wide_z()?;
    }

    let mut items = Vec::with_capacity(cdit as usize);
    for _ in 0..cdit {
        cur.align_dword()?;
        let (istyle, iex_style, ix, iy, icx, icy, iid) = if is_ex {
            let _help_id = cur.read_u32()?;
            let iex_style = cur.read_u32()?;
            let istyle = cur.read_u32()?;
            let ix = cur.read_i16()?;
            let iy = cur.read_i16()?;
            let icx = cur.read_i16()?;
            let icy = cur.read_i16()?;
            let iid = cur.read_u32()?;
            (istyle, iex_style, ix, iy, icx, icy, iid)
        } else {
            let istyle = cur.read_u32()?;
            let iex_style = cur.read_u32()?;
            let ix = cur.read_i16()?;
            let iy = cur.read_i16()?;
            let icx = cur.read_i16()?;
            let icy = cur.read_i16()?;
            let iid = cur.read_u16()? as u32;
            (istyle, iex_style, ix, iy, icx, icy, iid)
        };

        let (icls_ord, icls_name) = cur.read_sz_or_ord()?;
        let item_class = match (icls_ord, icls_name.is_empty()) {
            (Some(ord), _) => TemplateClass::Atom(ord),
            (None, false) => TemplateClass::Name(icls_name),
            (None, true) => return Err(()),
        };

        let (_ititle_ord, ititle) = cur.read_sz_or_ord()?;

        let extra_count = cur.read_u16()? as usize;
        let payload_len = if extra_count == 0 {
            0
        } else if is_ex {
            // DLGITEMTEMPLATEEX.extraCount counts only the creation-data bytes following this WORD.
            extra_count
        } else {
            // Standard DLGITEMTEMPLATE nonzero creation-data size includes the 2-byte size WORD.
            if extra_count < 2 {
                return Err(());
            }
            extra_count - 2
        };
        let extra_ptr = if payload_len > 0 {
            cur.check(payload_len)?;
            let p = cur.base.add(cur.off) as usize;
            cur.off += payload_len;
            p
        } else {
            0
        };

        items.push(ParsedItem {
            style: istyle,
            ex_style: iex_style,
            x: ix,
            y: iy,
            cx: icx,
            cy: icy,
            id: iid,
            class: item_class,
            title: ititle,
            extra_ptr,
        });
    }

    Ok(ParsedTemplate {
        style,
        ex_style,
        x,
        y,
        cx,
        cy,
        class,
        title,
        items,
    })
}

#[inline]
fn dlu_x(v: i16) -> i32 {
    // Standard 8x16 dialog base units: x * 8 / 4 = x * 2.
    (v as i32) * 2
}

#[inline]
fn dlu_y(v: i16) -> i32 {
    // Standard 8x16 dialog base units: y * 16 / 8 = y * 2.
    (v as i32) * 2
}

// ---------------------------------------------------------------------------
// Dialog creation, DefDlgProc, modal loop, and EndDialog
// ---------------------------------------------------------------------------

fn returns_direct_result(msg: u32) -> bool {
    matches!(
        msg,
        wm::INITDIALOG
            | wm::CTLCOLORMSGBOX..=wm::CTLCOLORSTATIC
            | wm::COMPAREITEM
            | wm::VKEYTOITEM
            | wm::CHARTOITEM
            | wm::QUERYDRAGICON
    )
}

unsafe fn first_focusable_control(hdlg: usize) -> usize {
    let children = window::children_of(hdlg);
    for &c in &children {
        if let Some(w) = window::get(c) {
            let d = w.data();
            if d.enabled && (d.style & ws::TABSTOP) != 0 {
                return c;
            }
        }
    }
    children.first().copied().unwrap_or(0)
}

unsafe fn def_dlg_proc_impl(hdlg: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let h = hdlg as usize;
    if !window::exists(h) {
        return 0;
    }

    let dlgproc = GetWindowLongPtrW(hdlg, DWLP_DLGPROC) as usize;
    // Clear DWLP_MSGRESULT before invoking the dialog procedure (Win32 contract).
    let _ = SetWindowLongPtrW(hdlg, DWLP_MSGRESULT, 0);

    if dlgproc != 0 {
        let handled = crate::call_guest8(
            dlgproc,
            h as u64,
            msg as u64,
            wparam as u64,
            lparam as u64,
            0,
            0,
            0,
            0,
        ) as isize;
        if handled != 0 {
            if returns_direct_result(msg) {
                return handled;
            }
            return GetWindowLongPtrW(hdlg, DWLP_MSGRESULT);
        }
    }

    match msg {
        wm::CLOSE => {
            let is_modal = dialogs()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .get(&h)
                .is_some_and(|s| s.modal);
            if is_modal {
                EndDialog(hdlg, IDCANCEL as isize);
            } else {
                DestroyWindow(hdlg);
            }
            0
        }
        wm::NCDESTROY => {
            forget_dialog(h);
            DefWindowProcW(hdlg, msg, wparam, lparam)
        }
        DM_GETDEFID => {
            let def_id = dialogs()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .get(&h)
                .map(|s| s.def_id)
                .unwrap_or(0);
            if def_id != 0 {
                ((DC_HASDEFID << 16) | def_id as usize) as LRESULT
            } else {
                0
            }
        }
        DM_SETDEFID => {
            if let Some(st) = dialogs().lock().unwrap_or_else(|e| e.into_inner()).get_mut(&h) {
                st.def_id = (wparam & 0xFFFF) as u16;
            }
            1
        }
        wm::NEXTDLGCTL => {
            let target = if (lparam & 0xFFFF) != 0 {
                wparam
            } else {
                let prev = (wparam != 0) as BOOL;
                GetNextDlgTabItem(hdlg, input::GetFocus(), prev) as usize
            };
            if target != 0 && window::exists(target) {
                input::set_focus(target);
            }
            1
        }
        wm::SETFOCUS => {
            let ctrl = first_focusable_control(h);
            if ctrl != 0 && ctrl != h {
                input::set_focus(ctrl);
            }
            0
        }
        _ => DefWindowProcW(hdlg, msg, wparam, lparam),
    }
}

pub unsafe extern "win64" fn DefDlgProcW(hdlg: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    def_dlg_proc_impl(hdlg, msg, wparam, lparam)
}

pub unsafe extern "win64" fn DefDlgProcA(hdlg: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        wm::SETTEXT if lparam != 0 => {
            let w = crate::strings::to_wide_z(&crate::strings::from_ansi(lparam as *const u8));
            def_dlg_proc_impl(hdlg, msg, wparam, w.as_ptr() as LPARAM)
        }
        _ => def_dlg_proc_impl(hdlg, msg, wparam, lparam),
    }
}

unsafe fn create_dialog_indirect_internal(
    hinstance: HINSTANCE,
    tmpl_ptr: *const u8,
    tmpl_limit: Option<usize>,
    hwnd_parent: HWND,
    dlg_proc: usize,
    init_param: LPARAM,
    modal: bool,
) -> HWND {
    ensure_dialog_classes();

    let Ok(tmpl) = parse_dialog_template(tmpl_ptr, tmpl_limit) else {
        set_error(e::INVALID_DATA);
        return std::ptr::null_mut();
    };

    let want_visible = (tmpl.style & ws::VISIBLE) != 0;
    let create_style = tmpl.style & !ws::VISIBLE;

    let cw = dlu_x(tmpl.cx).max(0);
    let ch = dlu_y(tmpl.cy).max(0);
    let mut rc = RECT::new(dlu_x(tmpl.x), dlu_y(tmpl.y), dlu_x(tmpl.x) + cw, dlu_y(tmpl.y) + ch);
    window::AdjustWindowRectEx(&mut rc, create_style, FALSE, tmpl.ex_style);

    let default_cls_w = crate::strings::to_wide_z("#32770");
    let custom_cls_w: Vec<u16>;
    let cls_ptr: LPCWSTR = match &tmpl.class {
        TemplateClass::DefaultDialog => default_cls_w.as_ptr(),
        TemplateClass::Atom(ord) => {
            if *ord == 32770 {
                default_cls_w.as_ptr()
            } else {
                *ord as usize as LPCWSTR
            }
        }
        TemplateClass::Name(units) => {
            custom_cls_w = units.iter().copied().chain(std::iter::once(0)).collect();
            custom_cls_w.as_ptr()
        }
    };

    let title_w: Vec<u16> = tmpl.title.iter().copied().chain(std::iter::once(0)).collect();

    let hdlg = CreateWindowExW(
        tmpl.ex_style,
        cls_ptr,
        title_w.as_ptr(),
        create_style,
        rc.left,
        rc.top,
        rc.width(),
        rc.height(),
        hwnd_parent,
        0,
        hinstance,
        std::ptr::null_mut(),
    );
    if hdlg.is_null() {
        return std::ptr::null_mut();
    }

    let h = hdlg as usize;
    // Ensure at least DLGWINDOWEXTRA bytes even for custom dialog classes.
    if let Some(w) = window::get(h) {
        let mut d = w.data();
        if d.extra.len() < DLGWINDOWEXTRA as usize {
            d.extra.resize(DLGWINDOWEXTRA as usize, 0);
        }
    }

    dialogs().lock().unwrap_or_else(|e| e.into_inner()).insert(
        h,
        DialogState {
            modal,
            ended: false,
            result: 0,
            def_id: 0,
        },
    );

    if dlg_proc != 0 {
        SetWindowLongPtrW(hdlg, DWLP_DLGPROC, dlg_proc as isize);
    }

    // Create child controls from template items.
    let mut def_id: u16 = 0;
    for item in &tmpl.items {
        let item_cls_w: Vec<u16>;
        let item_cls_ptr: LPCWSTR = match &item.class {
            TemplateClass::Atom(ord) => {
                if let Some(name) = predefined_control_class_name(*ord) {
                    if *ord == 0x0080 && (item.style & BS_TYPEMASK) == BS_DEFPUSHBUTTON && def_id == 0 {
                        def_id = item.id as u16;
                    }
                    item_cls_w = crate::strings::to_wide_z(name);
                    item_cls_w.as_ptr()
                } else {
                    *ord as usize as LPCWSTR
                }
            }
            TemplateClass::Name(units) => {
                let s = String::from_utf16_lossy(units);
                if s.eq_ignore_ascii_case("Button") && (item.style & BS_TYPEMASK) == BS_DEFPUSHBUTTON && def_id == 0 {
                    def_id = item.id as u16;
                }
                item_cls_w = units.iter().copied().chain(std::iter::once(0)).collect();
                item_cls_w.as_ptr()
            }
            TemplateClass::DefaultDialog => default_cls_w.as_ptr(),
        };

        let item_title_w: Vec<u16> = item.title.iter().copied().chain(std::iter::once(0)).collect();
        let child = CreateWindowExW(
            item.ex_style,
            item_cls_ptr,
            item_title_w.as_ptr(),
            item.style | ws::CHILD,
            dlu_x(item.x),
            dlu_y(item.y),
            dlu_x(item.cx),
            dlu_y(item.cy),
            hdlg,
            item.id as usize,
            hinstance,
            item.extra_ptr as LPVOID,
        );
        if child.is_null() {
            log::warn!("dialog {h:#x}: failed to create child control id={}", item.id);
        }
    }

    if def_id != 0 {
        if let Some(st) = dialogs().lock().unwrap_or_else(|e| e.into_inner()).get_mut(&h) {
            st.def_id = def_id;
        }
    }

    let first_ctrl = first_focusable_control(h);
    let init_ret = send_message(h, wm::INITDIALOG, first_ctrl, init_param);

    if !window::exists(h) {
        return std::ptr::null_mut();
    }

    let ended = dialogs()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get(&h)
        .is_some_and(|s| s.ended);

    if !ended {
        if init_ret != 0 && first_ctrl != 0 && window::exists(first_ctrl) {
            input::set_focus(first_ctrl);
        }
        if want_visible || modal {
            ShowWindow(hdlg, SW_SHOWNORMAL);
        }
    }

    hdlg
}

unsafe fn find_dialog_resource_w(hinstance: HINSTANCE, tmpl_name: LPCWSTR) -> Option<(*const u8, usize)> {
    if hinstance.is_null() && crate::try_loader().is_none() {
        set_error(e::RESOURCE_TYPE_NOT_FOUND);
        return None;
    }
    let hres = crate::kernel32::module::FindResourceW(hinstance, tmpl_name, RT_DIALOG as LPCWSTR);
    if hres.is_null() {
        return None;
    }
    let sz = crate::kernel32::module::SizeofResource(hinstance, hres) as usize;
    let hglob = crate::kernel32::module::LoadResource(hinstance, hres);
    if hglob.is_null() {
        return None;
    }
    let ptr = crate::kernel32::module::LockResource(hglob) as *const u8;
    if ptr.is_null() {
        set_error(e::INVALID_DATA);
        return None;
    }
    Some((ptr, sz))
}

unsafe fn find_dialog_resource_a(hinstance: HINSTANCE, tmpl_name: LPCSTR) -> Option<(*const u8, usize)> {
    if hinstance.is_null() && crate::try_loader().is_none() {
        set_error(e::RESOURCE_TYPE_NOT_FOUND);
        return None;
    }
    let hres = crate::kernel32::module::FindResourceA(hinstance, tmpl_name, RT_DIALOG as LPCSTR);
    if hres.is_null() {
        return None;
    }
    let sz = crate::kernel32::module::SizeofResource(hinstance, hres) as usize;
    let hglob = crate::kernel32::module::LoadResource(hinstance, hres);
    if hglob.is_null() {
        return None;
    }
    let ptr = crate::kernel32::module::LockResource(hglob) as *const u8;
    if ptr.is_null() {
        set_error(e::INVALID_DATA);
        return None;
    }
    Some((ptr, sz))
}

unsafe fn run_modal_dialog(
    hinstance: HINSTANCE,
    tmpl_ptr: *const u8,
    tmpl_limit: Option<usize>,
    hwnd_parent: HWND,
    dlg_proc: usize,
    init_param: LPARAM,
) -> isize {
    let owner_h = hwnd_parent as usize;
    let disabled_owner = if owner_h != 0 && window::exists(owner_h) {
        // EnableWindow returns nonzero if the window was ALREADY disabled.
        EnableWindow(hwnd_parent, FALSE) == FALSE
    } else {
        false
    };

    let hdlg = create_dialog_indirect_internal(
        hinstance,
        tmpl_ptr,
        tmpl_limit,
        hwnd_parent,
        dlg_proc,
        init_param,
        true,
    );
    if hdlg.is_null() {
        if disabled_owner && window::exists(owner_h) {
            EnableWindow(hwnd_parent, TRUE);
        }
        return -1;
    }

    let h = hdlg as usize;
    let mut msg = MSG::default();
    loop {
        let ended = dialogs()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&h)
            .map(|s| s.ended)
            .unwrap_or(true);
        if ended || !window::exists(h) {
            break;
        }
        let r = GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0);
        if r <= 0 {
            if r == 0 {
                // Re-post WM_QUIT so the outer application loop sees it.
                PostQuitMessage(msg.wparam as i32);
            }
            break;
        }
        let still_ended = dialogs()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&h)
            .map(|s| s.ended)
            .unwrap_or(true);
        if still_ended || !window::exists(h) {
            break;
        }
        if IsDialogMessageW(hdlg, &mut msg) == FALSE {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    let result = dialogs()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get(&h)
        .map(|s| s.result)
        .unwrap_or(0);

    // Re-enable owner before destroying the modal dialog (Win32 order).
    if disabled_owner && window::exists(owner_h) {
        EnableWindow(hwnd_parent, TRUE);
    }

    if window::exists(h) {
        DestroyWindow(hdlg);
    }
    forget_dialog(h);
    result
}

pub unsafe extern "win64" fn CreateDialogIndirectParamW(
    hinstance: HINSTANCE,
    lp_template: *const u8,
    hwnd_parent: HWND,
    lp_dialog_func: usize,
    lparam_init: LPARAM,
) -> HWND {
    create_dialog_indirect_internal(hinstance, lp_template, None, hwnd_parent, lp_dialog_func, lparam_init, false)
}

pub unsafe extern "win64" fn CreateDialogIndirectParamA(
    hinstance: HINSTANCE,
    lp_template: *const u8,
    hwnd_parent: HWND,
    lp_dialog_func: usize,
    lparam_init: LPARAM,
) -> HWND {
    create_dialog_indirect_internal(hinstance, lp_template, None, hwnd_parent, lp_dialog_func, lparam_init, false)
}

pub unsafe extern "win64" fn CreateDialogParamW(
    hinstance: HINSTANCE,
    lp_template_name: LPCWSTR,
    hwnd_parent: HWND,
    lp_dialog_func: usize,
    dw_init_param: LPARAM,
) -> HWND {
    let Some((ptr, sz)) = find_dialog_resource_w(hinstance, lp_template_name) else {
        return std::ptr::null_mut();
    };
    create_dialog_indirect_internal(hinstance, ptr, Some(sz), hwnd_parent, lp_dialog_func, dw_init_param, false)
}

pub unsafe extern "win64" fn CreateDialogParamA(
    hinstance: HINSTANCE,
    lp_template_name: LPCSTR,
    hwnd_parent: HWND,
    lp_dialog_func: usize,
    dw_init_param: LPARAM,
) -> HWND {
    let Some((ptr, sz)) = find_dialog_resource_a(hinstance, lp_template_name) else {
        return std::ptr::null_mut();
    };
    create_dialog_indirect_internal(hinstance, ptr, Some(sz), hwnd_parent, lp_dialog_func, dw_init_param, false)
}

pub unsafe extern "win64" fn DialogBoxIndirectParamW(
    hinstance: HINSTANCE,
    h_dialog_template: *const u8,
    hwnd_parent: HWND,
    lp_dialog_func: usize,
    dw_init_param: LPARAM,
) -> isize {
    run_modal_dialog(hinstance, h_dialog_template, None, hwnd_parent, lp_dialog_func, dw_init_param)
}

pub unsafe extern "win64" fn DialogBoxIndirectParamA(
    hinstance: HINSTANCE,
    h_dialog_template: *const u8,
    hwnd_parent: HWND,
    lp_dialog_func: usize,
    dw_init_param: LPARAM,
) -> isize {
    run_modal_dialog(hinstance, h_dialog_template, None, hwnd_parent, lp_dialog_func, dw_init_param)
}

pub unsafe extern "win64" fn DialogBoxParamW(
    hinstance: HINSTANCE,
    lp_template_name: LPCWSTR,
    hwnd_parent: HWND,
    lp_dialog_func: usize,
    dw_init_param: LPARAM,
) -> isize {
    let Some((ptr, sz)) = find_dialog_resource_w(hinstance, lp_template_name) else {
        return -1;
    };
    run_modal_dialog(hinstance, ptr, Some(sz), hwnd_parent, lp_dialog_func, dw_init_param)
}

pub unsafe extern "win64" fn DialogBoxParamA(
    hinstance: HINSTANCE,
    lp_template_name: LPCSTR,
    hwnd_parent: HWND,
    lp_dialog_func: usize,
    dw_init_param: LPARAM,
) -> isize {
    let Some((ptr, sz)) = find_dialog_resource_a(hinstance, lp_template_name) else {
        return -1;
    };
    run_modal_dialog(hinstance, ptr, Some(sz), hwnd_parent, lp_dialog_func, dw_init_param)
}

pub unsafe extern "win64" fn EndDialog(hdlg: HWND, n_result: isize) -> BOOL {
    let h = hdlg as usize;
    let Some(w) = window::get(h) else {
        set_error(e::INVALID_WINDOW_HANDLE);
        return FALSE;
    };
    let tid = w.tid;
    {
        let mut map = dialogs().lock().unwrap_or_else(|e| e.into_inner());
        let Some(st) = map.get_mut(&h) else {
            set_error(e::INVALID_WINDOW_HANDLE);
            return FALSE;
        };
        st.ended = true;
        st.result = n_result;
    }
    if window::get(h).is_some_and(|w| w.data().visible) {
        ShowWindow(hdlg, SW_HIDE);
    }
    // Wake the modal loop if it is blocked in GetMessageW.
    if let Some(q) = message::queue_of(tid) {
        q.post(MSG { hwnd: h, message: wm::NULL, ..Default::default() });
    }
    TRUE
}

// ---------------------------------------------------------------------------
// Dialog item helpers
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn GetDlgItem(hdlg: HWND, id: i32) -> HWND {
    let h = hdlg as usize;
    if !window::exists(h) {
        set_error(e::INVALID_WINDOW_HANDLE);
        return std::ptr::null_mut();
    }
    for c in window::children_of(h) {
        if let Some(w) = window::get(c) {
            if w.data().id as i32 == id {
                return c as HWND;
            }
        }
    }
    set_error(e::CONTROL_ID_NOT_FOUND);
    std::ptr::null_mut()
}

pub unsafe extern "win64" fn GetDlgCtrlID(hwnd: HWND) -> i32 {
    let Some(w) = window::get(hwnd as usize) else {
        set_error(e::INVALID_WINDOW_HANDLE);
        return 0;
    };
    let id = w.data().id as i32;
    id
}

pub unsafe extern "win64" fn SetDlgItemTextW(hdlg: HWND, id: i32, text: LPCWSTR) -> BOOL {
    let item = GetDlgItem(hdlg, id);
    if item.is_null() {
        return FALSE;
    }
    SetWindowTextW(item, text)
}

pub unsafe extern "win64" fn SetDlgItemTextA(hdlg: HWND, id: i32, text: LPCSTR) -> BOOL {
    let item = GetDlgItem(hdlg, id);
    if item.is_null() {
        return FALSE;
    }
    SetWindowTextA(item, text)
}

pub unsafe extern "win64" fn GetDlgItemTextW(hdlg: HWND, id: i32, buf: LPWSTR, cch_max: i32) -> u32 {
    if buf.is_null() || cch_max <= 0 {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    let item = GetDlgItem(hdlg, id);
    if item.is_null() {
        buf.write_unaligned(0);
        return 0;
    }
    GetWindowTextW(item, buf, cch_max).max(0) as u32
}

pub unsafe extern "win64" fn GetDlgItemTextA(hdlg: HWND, id: i32, buf: LPSTR, cch_max: i32) -> u32 {
    if buf.is_null() || cch_max <= 0 {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    let item = GetDlgItem(hdlg, id);
    if item.is_null() {
        buf.write_unaligned(0);
        return 0;
    }
    GetWindowTextA(item, buf, cch_max).max(0) as u32
}

pub unsafe extern "win64" fn SetDlgItemInt(hdlg: HWND, id: i32, value: u32, signed: BOOL) -> BOOL {
    let s = if signed != 0 {
        format!("{}", value as i32)
    } else {
        format!("{value}")
    };
    let w = crate::strings::to_wide_z(&s);
    SetDlgItemTextW(hdlg, id, w.as_ptr())
}

pub unsafe extern "win64" fn GetDlgItemInt(
    hdlg: HWND,
    id: i32,
    translated: *mut BOOL,
    signed: BOOL,
) -> u32 {
    if !translated.is_null() {
        translated.write_unaligned(FALSE);
    }
    let item = GetDlgItem(hdlg, id);
    if item.is_null() {
        return 0;
    }
    let mut buf = [0u16; 64];
    let n = GetWindowTextW(item, buf.as_mut_ptr(), buf.len() as i32);
    if n <= 0 {
        return 0;
    }
    let s = String::from_utf16_lossy(&buf[..n as usize]);
    let trimmed = s.trim();
    if signed != 0 {
        if let Ok(v) = trimmed.parse::<i32>() {
            if !translated.is_null() {
                translated.write_unaligned(TRUE);
            }
            return v as u32;
        }
    } else if let Ok(v) = trimmed.parse::<u32>() {
        if !translated.is_null() {
            translated.write_unaligned(TRUE);
        }
        return v;
    }
    0
}

pub unsafe extern "win64" fn SendDlgItemMessageW(
    hdlg: HWND,
    id: i32,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let item = GetDlgItem(hdlg, id);
    if item.is_null() {
        return 0;
    }
    SendMessageW(item, msg, wparam, lparam)
}

pub unsafe extern "win64" fn SendDlgItemMessageA(
    hdlg: HWND,
    id: i32,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let item = GetDlgItem(hdlg, id);
    if item.is_null() {
        return 0;
    }
    SendMessageA(item, msg, wparam, lparam)
}

pub unsafe extern "win64" fn CheckDlgButton(hdlg: HWND, id_button: i32, u_check: u32) -> BOOL {
    let item = GetDlgItem(hdlg, id_button);
    if item.is_null() {
        return FALSE;
    }
    SendMessageW(item, wm::BM_SETCHECK, u_check as WPARAM, 0);
    TRUE
}

pub unsafe extern "win64" fn IsDlgButtonChecked(hdlg: HWND, id_button: i32) -> u32 {
    let item = GetDlgItem(hdlg, id_button);
    if item.is_null() {
        return 0;
    }
    SendMessageW(item, wm::BM_GETCHECK, 0, 0) as u32
}

pub unsafe extern "win64" fn GetNextDlgTabItem(hdlg: HWND, hctl: HWND, b_previous: BOOL) -> HWND {
    let h = hdlg as usize;
    if !window::exists(h) {
        set_error(e::INVALID_WINDOW_HANDLE);
        return std::ptr::null_mut();
    }
    let tab_stops: Vec<usize> = window::children_of(h)
        .into_iter()
        .filter(|&c| {
            window::get(c).is_some_and(|w| {
                let d = w.data();
                d.enabled && (d.style & ws::TABSTOP) != 0
            })
        })
        .collect();
    if tab_stops.is_empty() {
        return std::ptr::null_mut();
    }
    let cur = hctl as usize;
    if cur == 0 {
        return if b_previous != 0 {
            *tab_stops.last().unwrap() as HWND
        } else {
            tab_stops[0] as HWND
        };
    }
    match tab_stops.iter().position(|&c| c == cur) {
        Some(idx) => {
            let next_idx = if b_previous != 0 {
                if idx == 0 { tab_stops.len() - 1 } else { idx - 1 }
            } else {
                (idx + 1) % tab_stops.len()
            };
            tab_stops[next_idx] as HWND
        }
        None => {
            if b_previous != 0 {
                *tab_stops.last().unwrap() as HWND
            } else {
                tab_stops[0] as HWND
            }
        }
    }
}

unsafe fn is_dialog_message_impl(hdlg: HWND, lp_msg: *mut MSG, unicode: bool) -> BOOL {
    let h = hdlg as usize;
    if !window::exists(h) || lp_msg.is_null() {
        return FALSE;
    }
    let msg = lp_msg.read_unaligned();
    if msg.hwnd != h && !window::is_child_of(msg.hwnd, h) {
        return FALSE;
    }

    if msg.message == wm::KEYDOWN && matches!(msg.wparam, VK_TAB | VK_RETURN | VK_ESCAPE) {
        // Query the target control via WM_GETDLGCODE outside any locks before
        // consuming dialog navigation, default, or cancel keys.
        let target = if msg.hwnd != 0 && msg.hwnd != h && window::exists(msg.hwnd) {
            msg.hwnd
        } else {
            let focus = input::GetFocus() as usize;
            if focus != 0 && (focus == h || window::is_child_of(focus, h)) {
                focus
            } else {
                msg.hwnd
            }
        };
        let dlg_code = if target != 0 {
            send_message(target, wm::GETDLGCODE, msg.wparam, lp_msg as LPARAM)
        } else {
            0
        };
        let wants_all = (dlg_code & (DLGC_WANTMESSAGE | DLGC_WANTALLKEYS)) != 0;

        if !wants_all {
            match msg.wparam {
                VK_TAB if (dlg_code & DLGC_WANTTAB) == 0 => {
                    let prev = input::shift_down() as BOOL;
                    let next = GetNextDlgTabItem(hdlg, input::GetFocus(), prev);
                    if !next.is_null() {
                        send_message(h, wm::NEXTDLGCTL, next as usize, 1);
                    }
                    return TRUE;
                }
                VK_RETURN => {
                    let def = send_message(h, DM_GETDEFID, 0, 0) as usize;
                    let def_id = if (def >> 16) == DC_HASDEFID {
                        (def & 0xFFFF) as i32
                    } else {
                        IDOK
                    };
                    let ctrl = GetDlgItem(hdlg, def_id);
                    send_message(
                        h,
                        wm::COMMAND,
                        ((BN_CLICKED & 0xFFFF) << 16) | (def_id as u16 as usize),
                        ctrl as LPARAM,
                    );
                    return TRUE;
                }
                VK_ESCAPE => {
                    let ctrl = GetDlgItem(hdlg, IDCANCEL);
                    send_message(
                        h,
                        wm::COMMAND,
                        ((BN_CLICKED & 0xFFFF) << 16) | (IDCANCEL as u16 as usize),
                        ctrl as LPARAM,
                    );
                    return TRUE;
                }
                _ => {}
            }
        }
    }

    TranslateMessage(lp_msg);
    if unicode {
        DispatchMessageW(lp_msg);
    } else {
        DispatchMessageA(lp_msg);
    }
    TRUE
}

pub unsafe extern "win64" fn IsDialogMessageW(hdlg: HWND, lp_msg: *mut MSG) -> BOOL {
    is_dialog_message_impl(hdlg, lp_msg, true)
}

pub unsafe extern "win64" fn IsDialogMessageA(hdlg: HWND, lp_msg: *mut MSG) -> BOOL {
    is_dialog_message_impl(hdlg, lp_msg, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicIsize, AtomicUsize, Ordering};

    fn push_u16(buf: &mut Vec<u8>, v: u16) {
        buf.extend_from_slice(&v.to_le_bytes());
    }
    fn push_u32(buf: &mut Vec<u8>, v: u32) {
        buf.extend_from_slice(&v.to_le_bytes());
    }
    fn push_wide_z(buf: &mut Vec<u8>, s: &str) {
        for u in s.encode_utf16() {
            push_u16(buf, u);
        }
        push_u16(buf, 0);
    }
    fn align4(buf: &mut Vec<u8>) {
        while buf.len() % 4 != 0 {
            buf.push(0);
        }
    }

    /// Build a real binary `DLGTEMPLATE` with Static, Edit, and Button items.
    fn build_std_template() -> Vec<u8> {
        let mut b = Vec::new();
        // Header: style, exStyle, cdit=3, x=10, y=10, cx=200, cy=120
        push_u32(&mut b, ws::POPUP | ws::CAPTION | ws::SYSMENU | DS_SETFONT);
        push_u32(&mut b, 0);
        push_u16(&mut b, 3);
        push_u16(&mut b, 10);
        push_u16(&mut b, 10);
        push_u16(&mut b, 200);
        push_u16(&mut b, 120);
        // menu = none, class = default, title = "Modeless Test"
        push_u16(&mut b, 0);
        push_u16(&mut b, 0);
        push_wide_z(&mut b, "Modeless Test");
        // DS_SETFONT: pointsize + typeface
        push_u16(&mut b, 9);
        push_wide_z(&mut b, "Segoe UI");

        // Item 1: Static label (id 100)
        align4(&mut b);
        push_u32(&mut b, ws::CHILD | ws::VISIBLE);
        push_u32(&mut b, 0);
        push_u16(&mut b, 8);
        push_u16(&mut b, 8);
        push_u16(&mut b, 80);
        push_u16(&mut b, 14);
        push_u16(&mut b, 100);
        push_u16(&mut b, 0xFFFF);
        push_u16(&mut b, 0x0082); // Static
        push_wide_z(&mut b, "Label:");
        push_u16(&mut b, 0); // extraCount

        // Item 2: Edit box (id 101, WS_TABSTOP)
        align4(&mut b);
        push_u32(&mut b, ws::CHILD | ws::VISIBLE | ws::TABSTOP);
        push_u32(&mut b, 0);
        push_u16(&mut b, 92);
        push_u16(&mut b, 8);
        push_u16(&mut b, 90);
        push_u16(&mut b, 14);
        push_u16(&mut b, 101);
        push_u16(&mut b, 0xFFFF);
        push_u16(&mut b, 0x0081); // Edit
        push_wide_z(&mut b, "initial");
        push_u16(&mut b, 0);

        // Item 3: Default OK Button (id IDOK=1, WS_TABSTOP | BS_DEFPUSHBUTTON)
        align4(&mut b);
        push_u32(&mut b, ws::CHILD | ws::VISIBLE | ws::TABSTOP | BS_DEFPUSHBUTTON);
        push_u32(&mut b, 0);
        push_u16(&mut b, 60);
        push_u16(&mut b, 80);
        push_u16(&mut b, 50);
        push_u16(&mut b, 14);
        push_u16(&mut b, IDOK as u16);
        push_u16(&mut b, 0xFFFF);
        push_u16(&mut b, 0x0080); // Button
        push_wide_z(&mut b, "OK");
        push_u16(&mut b, 0);

        b
    }

    /// Build a real binary `DLGTEMPLATEEX` with Edit, CheckBox Button, and Cancel Button.
    fn build_ex_template() -> Vec<u8> {
        let mut b = Vec::new();
        push_u16(&mut b, 1); // dlgVer
        push_u16(&mut b, 0xFFFF); // signature
        push_u32(&mut b, 0); // helpID
        push_u32(&mut b, 0); // exStyle
        push_u32(&mut b, ws::POPUP | ws::CAPTION | DS_SHELLFONT); // style
        push_u16(&mut b, 3); // cDlgItems
        push_u16(&mut b, 20);
        push_u16(&mut b, 20);
        push_u16(&mut b, 220);
        push_u16(&mut b, 140);
        push_u16(&mut b, 0); // menu
        push_u16(&mut b, 0); // class
        push_wide_z(&mut b, "Modal Ex Dialog");
        // DS_SHELLFONT font block: pointsize, weight, italic, charset, typeface
        push_u16(&mut b, 8);
        push_u16(&mut b, 400);
        b.push(0); // italic
        b.push(1); // DEFAULT_CHARSET
        push_wide_z(&mut b, "MS Shell Dlg 2");

        // Item 1: Edit (id 201)
        align4(&mut b);
        push_u32(&mut b, 0); // helpID
        push_u32(&mut b, 0); // exStyle
        push_u32(&mut b, ws::CHILD | ws::VISIBLE | ws::TABSTOP);
        push_u16(&mut b, 10);
        push_u16(&mut b, 10);
        push_u16(&mut b, 100);
        push_u16(&mut b, 14);
        push_u32(&mut b, 201); // 32-bit id
        push_u16(&mut b, 0xFFFF);
        push_u16(&mut b, 0x0081); // Edit
        push_wide_z(&mut b, "42");
        push_u16(&mut b, 0);

        // Item 2: Button (id 202, named class "Button")
        align4(&mut b);
        push_u32(&mut b, 0);
        push_u32(&mut b, 0);
        push_u32(&mut b, ws::CHILD | ws::VISIBLE | ws::TABSTOP);
        push_u16(&mut b, 10);
        push_u16(&mut b, 30);
        push_u16(&mut b, 80);
        push_u16(&mut b, 14);
        push_u32(&mut b, 202);
        push_wide_z(&mut b, "Button");
        push_wide_z(&mut b, "Enable");
        push_u16(&mut b, 0);

        // Item 3: Default OK Button (id IDOK=1)
        align4(&mut b);
        push_u32(&mut b, 0);
        push_u32(&mut b, 0);
        push_u32(&mut b, ws::CHILD | ws::VISIBLE | ws::TABSTOP | BS_DEFPUSHBUTTON);
        push_u16(&mut b, 10);
        push_u16(&mut b, 60);
        push_u16(&mut b, 50);
        push_u16(&mut b, 14);
        push_u32(&mut b, IDOK as u32);
        push_u16(&mut b, 0xFFFF);
        push_u16(&mut b, 0x0080); // Button
        push_wide_z(&mut b, "Confirm");
        push_u16(&mut b, 0);

        b
    }

    static INIT_PARAM_SEEN: AtomicIsize = AtomicIsize::new(0);
    static INIT_WPARAM_SEEN: AtomicUsize = AtomicUsize::new(0);
    static CUSTOM_MSG_SEEN: AtomicUsize = AtomicUsize::new(0);

    unsafe extern "win64" fn modeless_dlg_proc(
        hdlg: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> isize {
        match msg {
            wm::INITDIALOG => {
                INIT_PARAM_SEEN.store(lparam, Ordering::SeqCst);
                INIT_WPARAM_SEEN.store(wparam, Ordering::SeqCst);
                SetWindowLongPtrW(hdlg, DWLP_USER, 0x1337_CAFE);
                1 // TRUE: let dialog focus wparam
            }
            m if m == wm::USER + 5 => {
                CUSTOM_MSG_SEEN.store(wparam, Ordering::SeqCst);
                SetWindowLongPtrW(hdlg, DWLP_MSGRESULT, 0x55AA);
                1 // handled -> DefDlgProc returns DWLP_MSGRESULT
            }
            _ => 0,
        }
    }

    static OWNER_WAS_DISABLED_IN_MODAL: std::sync::atomic::AtomicBool =
        std::sync::atomic::AtomicBool::new(false);
    static MODAL_OWNER_HWND: AtomicUsize = AtomicUsize::new(0);

    unsafe extern "win64" fn modal_dlg_proc(
        hdlg: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> isize {
        match msg {
            wm::INITDIALOG => {
                INIT_PARAM_SEEN.store(lparam, Ordering::SeqCst);
                let owner = MODAL_OWNER_HWND.load(Ordering::SeqCst) as HWND;
                if !owner.is_null() {
                    OWNER_WAS_DISABLED_IN_MODAL
                        .store(window::IsWindowEnabled(owner) == FALSE, Ordering::SeqCst);
                }
                // Post VK_RETURN to trigger IsDialogMessageW -> WM_COMMAND(IDOK).
                message::PostMessageW(hdlg, wm::KEYDOWN, VK_RETURN, 0);
                1
            }
            wm::COMMAND => {
                let id = (wparam & 0xFFFF) as i32;
                if id == IDOK {
                    EndDialog(hdlg, 777);
                    return 1;
                }
                0
            }
            _ => 0,
        }
    }

    #[test]
    fn modeless_std_template_controls_items_and_dwlp_msgresult() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        assert!(!crate::macos::active(), "unit tests must stay headless");
        std::thread::spawn(|| unsafe {
            INIT_PARAM_SEEN.store(0, Ordering::SeqCst);
            INIT_WPARAM_SEEN.store(0, Ordering::SeqCst);
            CUSTOM_MSG_SEEN.store(0, Ordering::SeqCst);

            let tmpl = build_std_template();
            let hdlg = CreateDialogIndirectParamW(
                std::ptr::null_mut(),
                tmpl.as_ptr(),
                std::ptr::null_mut(),
                modeless_dlg_proc as *const () as usize,
                0x4242,
            );
            assert!(!hdlg.is_null());
            assert_eq!(INIT_PARAM_SEEN.load(Ordering::SeqCst), 0x4242);
            assert_eq!(GetWindowLongPtrW(hdlg, DWLP_USER), 0x1337_CAFE);

            // Dialog title parsed from template.
            let mut title_buf = [0u16; 32];
            let tlen = GetWindowTextW(hdlg, title_buf.as_mut_ptr(), 32);
            assert_eq!(String::from_utf16_lossy(&title_buf[..tlen as usize]), "Modeless Test");

            // Child controls exist with expected IDs and initial text.
            let h_static = GetDlgItem(hdlg, 100);
            let h_edit = GetDlgItem(hdlg, 101);
            let h_ok = GetDlgItem(hdlg, IDOK);
            assert!(!h_static.is_null());
            assert!(!h_edit.is_null());
            assert!(!h_ok.is_null());
            assert_eq!(GetDlgCtrlID(h_edit), 101);
            assert_eq!(INIT_WPARAM_SEEN.load(Ordering::SeqCst), h_edit as usize);
            assert_eq!(input::GetFocus(), h_edit);

            // GetDlgItemTextW / SetDlgItemTextW / SetDlgItemTextA / GetDlgItemTextA.
            let mut wbuf = [0u16; 32];
            let n = GetDlgItemTextW(hdlg, 101, wbuf.as_mut_ptr(), 32);
            assert_eq!(String::from_utf16_lossy(&wbuf[..n as usize]), "initial");

            let updated = crate::strings::to_wide_z("updated text");
            assert_eq!(SetDlgItemTextW(hdlg, 101, updated.as_ptr()), TRUE);
            let n2 = GetDlgItemTextW(hdlg, 101, wbuf.as_mut_ptr(), 32);
            assert_eq!(String::from_utf16_lossy(&wbuf[..n2 as usize]), "updated text");

            let ansi_text = b"ansi value\0";
            assert_eq!(SetDlgItemTextA(hdlg, 101, ansi_text.as_ptr()), TRUE);
            let mut abuf = [0u8; 32];
            let na = GetDlgItemTextA(hdlg, 101, abuf.as_mut_ptr(), 32);
            assert_eq!(&abuf[..na as usize], b"ansi value");

            // SetDlgItemInt / GetDlgItemInt (signed & unsigned).
            assert_eq!(SetDlgItemInt(hdlg, 101, (-123i32) as u32, TRUE), TRUE);
            let mut ok = FALSE;
            assert_eq!(GetDlgItemInt(hdlg, 101, &mut ok, TRUE) as i32, -123);
            assert_eq!(ok, TRUE);

            assert_eq!(SetDlgItemInt(hdlg, 101, 4000000000u32, FALSE), TRUE);
            assert_eq!(GetDlgItemInt(hdlg, 101, &mut ok, FALSE), 4000000000u32);
            assert_eq!(ok, TRUE);

            // CheckDlgButton / IsDlgButtonChecked / SendDlgItemMessageW.
            assert_eq!(IsDlgButtonChecked(hdlg, IDOK), 0);
            assert_eq!(CheckDlgButton(hdlg, IDOK, 1), TRUE);
            assert_eq!(IsDlgButtonChecked(hdlg, IDOK), 1);
            assert_eq!(SendDlgItemMessageW(hdlg, IDOK, wm::BM_GETCHECK, 0, 0), 1);

            // GetNextDlgTabItem cycles Edit <-> OK button (Static has no WS_TABSTOP).
            assert_eq!(GetNextDlgTabItem(hdlg, h_edit, FALSE), h_ok);
            assert_eq!(GetNextDlgTabItem(hdlg, h_ok, FALSE), h_edit);
            assert_eq!(GetNextDlgTabItem(hdlg, h_edit, TRUE), h_ok);

            // DWLP_MSGRESULT returned when DLGPROC returns nonzero.
            let res = SendMessageW(hdlg, wm::USER + 5, 0x99, 0);
            assert_eq!(CUSTOM_MSG_SEEN.load(Ordering::SeqCst), 0x99);
            assert_eq!(res, 0x55AA);

            assert_eq!(DestroyWindow(hdlg), TRUE);
            assert_eq!(window::IsWindow(hdlg), FALSE);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn modal_ex_template_disables_owner_and_returns_end_dialog_result() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        assert!(!crate::macos::active(), "unit tests must stay headless");
        std::thread::spawn(|| unsafe {
            INIT_PARAM_SEEN.store(0, Ordering::SeqCst);
            OWNER_WAS_DISABLED_IN_MODAL.store(false, Ordering::SeqCst);

            // Create a headless owner window.
            let owner_cls = crate::strings::to_wide_z("Aw2ModalOwnerCls");
            let wc = class::WNDCLASS {
                style: 0,
                wndproc: super::super::WNDPROC_HANDLE_DEF_WINDOW_W,
                cls_extra: 0,
                wnd_extra: 0,
                instance: 0,
                icon: 0,
                cursor: 0,
                background: 0,
                menu_name: 0,
                class_name: owner_cls.as_ptr() as usize,
            };
            let _ = class::RegisterClassW(&wc);
            let owner = CreateWindowExW(
                0,
                owner_cls.as_ptr(),
                std::ptr::null(),
                ws::POPUP,
                0,
                0,
                200,
                200,
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!owner.is_null());
            assert_eq!(window::IsWindowEnabled(owner), TRUE);
            MODAL_OWNER_HWND.store(owner as usize, Ordering::SeqCst);

            let tmpl = build_ex_template();
            let res = DialogBoxIndirectParamW(
                std::ptr::null_mut(),
                tmpl.as_ptr(),
                owner,
                modal_dlg_proc as *const () as usize,
                0x9876,
            );
            assert_eq!(res, 777);
            assert_eq!(INIT_PARAM_SEEN.load(Ordering::SeqCst), 0x9876);
            assert!(OWNER_WAS_DISABLED_IN_MODAL.load(Ordering::SeqCst));
            // Owner must be re-enabled after modal dialog exits.
            assert_eq!(window::IsWindowEnabled(owner), TRUE);

            MODAL_OWNER_HWND.store(0, Ordering::SeqCst);
            assert_eq!(DestroyWindow(owner), TRUE);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn malformed_templates_and_missing_resources_return_honest_errors() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        std::thread::spawn(|| unsafe {
            // Null template -> NULL / -1 with ERROR_INVALID_DATA.
            let h_null = CreateDialogIndirectParamW(
                std::ptr::null_mut(),
                std::ptr::null(),
                std::ptr::null_mut(),
                0,
                0,
            );
            assert!(h_null.is_null());
            assert_eq!(crate::teb::last_error(), e::INVALID_DATA);

            // Corrupt DLGTEMPLATEEX (signature=0xFFFF with bad dlgVer=2).
            let bad_ex = [2u8, 0, 0xFF, 0xFF, 0, 0, 0, 0];
            let r_bad = DialogBoxIndirectParamW(
                std::ptr::null_mut(),
                bad_ex.as_ptr(),
                std::ptr::null_mut(),
                0,
                0,
            );
            assert_eq!(r_bad, -1);
            assert_eq!(crate::teb::last_error(), e::INVALID_DATA);

            // Missing resource -> NULL / -1 with a Win32 resource-not-found error.
            let r_res = DialogBoxParamW(
                std::ptr::null_mut(),
                101usize as LPCWSTR,
                std::ptr::null_mut(),
                0,
                0,
            );
            assert_eq!(r_res, -1);
            let res_err = crate::teb::last_error();
            assert!(
                res_err == e::RESOURCE_TYPE_NOT_FOUND
                    || res_err == crate::types::err::ERROR_RESOURCE_DATA_NOT_FOUND,
                "unexpected missing-resource error: {res_err}"
            );

            // EndDialog on invalid HWND -> FALSE + ERROR_INVALID_WINDOW_HANDLE.
            assert_eq!(EndDialog(0xDEAD_BEEFusize as HWND, 1), FALSE);
            assert_eq!(crate::teb::last_error(), e::INVALID_WINDOW_HANDLE);
        })
        .join()
        .unwrap();
    }

    static DLGCODE_REPLY: AtomicIsize = AtomicIsize::new(0);
    static DLGCODE_LAST_WPARAM: AtomicUsize = AtomicUsize::new(0);
    static DLGCODE_LAST_LPARAM: AtomicIsize = AtomicIsize::new(0);
    static CTRL_LAST_KEYDOWN: AtomicUsize = AtomicUsize::new(0);
    static CTRL_KEYDOWN_COUNT: AtomicUsize = AtomicUsize::new(0);
    static DLG_LAST_CMD_ID: AtomicUsize = AtomicUsize::new(0);
    static DLG_CMD_COUNT: AtomicUsize = AtomicUsize::new(0);

    unsafe extern "win64" fn dlgcode_test_ctrl_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match msg {
            wm::GETDLGCODE => {
                DLGCODE_LAST_WPARAM.store(wparam, Ordering::SeqCst);
                DLGCODE_LAST_LPARAM.store(lparam, Ordering::SeqCst);
                DLGCODE_REPLY.load(Ordering::SeqCst)
            }
            wm::KEYDOWN => {
                CTRL_LAST_KEYDOWN.store(wparam, Ordering::SeqCst);
                CTRL_KEYDOWN_COUNT.fetch_add(1, Ordering::SeqCst);
                0
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }

    unsafe extern "win64" fn dlgcode_test_dlg_proc(
        _hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        _lparam: LPARAM,
    ) -> isize {
        match msg {
            wm::INITDIALOG => 1,
            wm::COMMAND => {
                DLG_LAST_CMD_ID.store(wparam & 0xFFFF, Ordering::SeqCst);
                DLG_CMD_COUNT.fetch_add(1, Ordering::SeqCst);
                1
            }
            _ => 0,
        }
    }

    #[test]
    fn is_dialog_message_honors_wm_getdlgcode_and_preserves_default_navigation() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        assert!(!crate::macos::active(), "unit tests must stay headless");
        std::thread::spawn(|| unsafe {
            let ctrl_proc_handle = super::super::register_typed_host_wndproc(dlgcode_test_ctrl_proc);
            let dlg_proc_va = dlgcode_test_dlg_proc as *const () as usize;
            crate::register_native_va(dlg_proc_va as u64, dlg_proc_va);

            let tmpl = build_std_template();
            let hdlg = CreateDialogIndirectParamW(
                std::ptr::null_mut(),
                tmpl.as_ptr(),
                std::ptr::null_mut(),
                dlg_proc_va,
                0,
            );
            assert!(!hdlg.is_null());
            let h_edit = GetDlgItem(hdlg, 101);
            let h_ok = GetDlgItem(hdlg, IDOK);
            assert!(!h_edit.is_null());
            assert!(!h_ok.is_null());

            // Subclass the Edit control with our headless WM_GETDLGCODE test wndproc (GWLP_WNDPROC = -4).
            let orig_edit_proc = SetWindowLongPtrW(h_edit, -4, ctrl_proc_handle as isize) as usize;
            assert_eq!(orig_edit_proc, super::super::WNDPROC_HANDLE_EDIT);

            let make_key_msg = |hwnd: HWND, vk: usize| MSG {
                hwnd: hwnd as usize,
                message: wm::KEYDOWN,
                wparam: vk,
                lparam: 0,
                time: 0,
                pt: POINT { x: 0, y: 0 },
                l_private: 0,
            };

            let reset_counters = || {
                DLGCODE_LAST_WPARAM.store(0, Ordering::SeqCst);
                DLGCODE_LAST_LPARAM.store(0, Ordering::SeqCst);
                CTRL_LAST_KEYDOWN.store(0, Ordering::SeqCst);
                CTRL_KEYDOWN_COUNT.store(0, Ordering::SeqCst);
                DLG_LAST_CMD_ID.store(0, Ordering::SeqCst);
                DLG_CMD_COUNT.store(0, Ordering::SeqCst);
            };

            // 1. DLGC_WANTTAB: Tab goes directly to the control without moving focus,
            //    while Return and Escape still trigger dialog IDOK / IDCANCEL commands.
            input::SetFocus(h_edit);
            DLGCODE_REPLY.store(DLGC_WANTTAB, Ordering::SeqCst);
            reset_counters();

            let mut msg_tab = make_key_msg(h_edit, VK_TAB);
            let msg_tab_ptr = &mut msg_tab as *mut MSG;
            assert_eq!(IsDialogMessageW(hdlg, msg_tab_ptr), TRUE);
            assert_eq!(DLGCODE_LAST_WPARAM.load(Ordering::SeqCst), VK_TAB);
            assert_eq!(DLGCODE_LAST_LPARAM.load(Ordering::SeqCst), msg_tab_ptr as LPARAM);
            assert_eq!(input::GetFocus(), h_edit, "DLGC_WANTTAB must not move focus");
            assert_eq!(CTRL_KEYDOWN_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(CTRL_LAST_KEYDOWN.load(Ordering::SeqCst), VK_TAB);
            assert_eq!(DLG_CMD_COUNT.load(Ordering::SeqCst), 0);

            reset_counters();
            let mut msg_ret = make_key_msg(h_edit, VK_RETURN);
            let msg_ret_ptr = &mut msg_ret as *mut MSG;
            assert_eq!(IsDialogMessageW(hdlg, msg_ret_ptr), TRUE);
            assert_eq!(DLGCODE_LAST_WPARAM.load(Ordering::SeqCst), VK_RETURN);
            assert_eq!(DLGCODE_LAST_LPARAM.load(Ordering::SeqCst), msg_ret_ptr as LPARAM);
            assert_eq!(CTRL_KEYDOWN_COUNT.load(Ordering::SeqCst), 0);
            assert_eq!(DLG_CMD_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(DLG_LAST_CMD_ID.load(Ordering::SeqCst), IDOK as usize);

            reset_counters();
            let mut msg_esc = make_key_msg(h_edit, VK_ESCAPE);
            let msg_esc_ptr = &mut msg_esc as *mut MSG;
            assert_eq!(IsDialogMessageW(hdlg, msg_esc_ptr), TRUE);
            assert_eq!(DLGCODE_LAST_WPARAM.load(Ordering::SeqCst), VK_ESCAPE);
            assert_eq!(DLGCODE_LAST_LPARAM.load(Ordering::SeqCst), msg_esc_ptr as LPARAM);
            assert_eq!(CTRL_KEYDOWN_COUNT.load(Ordering::SeqCst), 0);
            assert_eq!(DLG_CMD_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(DLG_LAST_CMD_ID.load(Ordering::SeqCst), IDCANCEL as usize);

            // 2. DLGC_WANTMESSAGE / DLGC_WANTALLKEYS: Tab, Return, and Escape all go
            //    straight to the control without changing focus or firing WM_COMMAND.
            input::SetFocus(h_edit);
            DLGCODE_REPLY.store(DLGC_WANTMESSAGE, Ordering::SeqCst);
            for vk in [VK_TAB, VK_RETURN, VK_ESCAPE] {
                reset_counters();
                let mut msg = make_key_msg(h_edit, vk);
                let msg_ptr = &mut msg as *mut MSG;
                assert_eq!(IsDialogMessageW(hdlg, msg_ptr), TRUE);
                assert_eq!(DLGCODE_LAST_WPARAM.load(Ordering::SeqCst), vk);
                assert_eq!(DLGCODE_LAST_LPARAM.load(Ordering::SeqCst), msg_ptr as LPARAM);
                assert_eq!(input::GetFocus(), h_edit, "DLGC_WANTMESSAGE must preserve focus for vk={vk:#x}");
                assert_eq!(CTRL_KEYDOWN_COUNT.load(Ordering::SeqCst), 1);
                assert_eq!(CTRL_LAST_KEYDOWN.load(Ordering::SeqCst), vk);
                assert_eq!(DLG_CMD_COUNT.load(Ordering::SeqCst), 0);
            }

            // 3. Ordinary control (DLGC_WANTARROWS | DLGC_WANTCHARS): default dialog
            //    navigation (Tab focus change, Return -> IDOK, Escape -> IDCANCEL) works.
            input::SetFocus(h_edit);
            DLGCODE_REPLY.store(DLGC_WANTARROWS | DLGC_WANTCHARS, Ordering::SeqCst);

            reset_counters();
            let mut msg_tab_def = make_key_msg(h_edit, VK_TAB);
            assert_eq!(IsDialogMessageW(hdlg, &mut msg_tab_def), TRUE);
            assert_eq!(input::GetFocus(), h_ok, "ordinary control Tab must advance focus to OK button");
            assert_eq!(CTRL_KEYDOWN_COUNT.load(Ordering::SeqCst), 0);

            input::SetFocus(h_edit);
            reset_counters();
            let mut msg_ret_def = make_key_msg(h_edit, VK_RETURN);
            assert_eq!(IsDialogMessageW(hdlg, &mut msg_ret_def), TRUE);
            assert_eq!(CTRL_KEYDOWN_COUNT.load(Ordering::SeqCst), 0);
            assert_eq!(DLG_CMD_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(DLG_LAST_CMD_ID.load(Ordering::SeqCst), IDOK as usize);

            reset_counters();
            let mut msg_esc_def = make_key_msg(h_edit, VK_ESCAPE);
            assert_eq!(IsDialogMessageW(hdlg, &mut msg_esc_def), TRUE);
            assert_eq!(CTRL_KEYDOWN_COUNT.load(Ordering::SeqCst), 0);
            assert_eq!(DLG_CMD_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(DLG_LAST_CMD_ID.load(Ordering::SeqCst), IDCANCEL as usize);

            assert_eq!(DestroyWindow(hdlg), TRUE);
        })
        .join()
        .unwrap();
    }

    fn build_std_header(style: u32, cdit: u16, title: &str) -> Vec<u8> {
        let mut b = Vec::new();
        push_u32(&mut b, style);
        push_u32(&mut b, 0);
        push_u16(&mut b, cdit);
        push_u16(&mut b, 0);
        push_u16(&mut b, 0);
        push_u16(&mut b, 120);
        push_u16(&mut b, 80);
        push_u16(&mut b, 0); // menu
        push_u16(&mut b, 0); // class
        push_wide_z(&mut b, title);
        b
    }

    fn build_ex_header(style: u32, cdit: u16, title: &str) -> Vec<u8> {
        let mut b = Vec::new();
        push_u16(&mut b, 1); // dlgVer
        push_u16(&mut b, 0xFFFF); // signature
        push_u32(&mut b, 0); // helpID
        push_u32(&mut b, 0); // exStyle
        push_u32(&mut b, style);
        push_u16(&mut b, cdit);
        push_u16(&mut b, 0);
        push_u16(&mut b, 0);
        push_u16(&mut b, 120);
        push_u16(&mut b, 80);
        push_u16(&mut b, 0); // menu
        push_u16(&mut b, 0); // class
        push_wide_z(&mut b, title);
        b
    }

    fn push_std_item(buf: &mut Vec<u8>, id: u16, cls: Option<&str>, title: &str, raw_extra_word: u16, payload: &[u8]) {
        align4(buf);
        push_u32(buf, ws::CHILD | ws::VISIBLE);
        push_u32(buf, 0);
        push_u16(buf, 4);
        push_u16(buf, 4);
        push_u16(buf, 40);
        push_u16(buf, 12);
        push_u16(buf, id);
        match cls {
            Some(name) => push_wide_z(buf, name),
            None => {
                push_u16(buf, 0xFFFF);
                push_u16(buf, 0x0082); // Static
            }
        }
        push_wide_z(buf, title);
        push_u16(buf, raw_extra_word);
        buf.extend_from_slice(payload);
    }

    fn push_ex_item(buf: &mut Vec<u8>, id: u32, cls: Option<&str>, title: &str, extra_count: u16, payload: &[u8]) {
        align4(buf);
        push_u32(buf, 0); // helpID
        push_u32(buf, 0); // exStyle
        push_u32(buf, ws::CHILD | ws::VISIBLE);
        push_u16(buf, 4);
        push_u16(buf, 4);
        push_u16(buf, 40);
        push_u16(buf, 12);
        push_u32(buf, id);
        match cls {
            Some(name) => push_wide_z(buf, name),
            None => {
                push_u16(buf, 0xFFFF);
                push_u16(buf, 0x0082); // Static
            }
        }
        push_wide_z(buf, title);
        push_u16(buf, extra_count);
        buf.extend_from_slice(payload);
    }

    #[test]
    fn dialog_template_ds_fixedsys_without_ds_setfont_has_no_font_block() {
        unsafe {
            // 1. Standard zero-control template with DS_FIXEDSYS alone (no DS_SETFONT).
            let std_zero = build_std_header(ws::POPUP | DS_FIXEDSYS, 0, "StdFixedZero");
            let parsed = parse_dialog_template(std_zero.as_ptr(), Some(std_zero.len()))
                .expect("zero-control DS_FIXEDSYS standard template must parse without font block");
            assert_eq!(String::from_utf16_lossy(&parsed.title), "StdFixedZero");
            assert!(parsed.items.is_empty());

            // 2. Standard 1-control template with DS_FIXEDSYS alone.
            let mut std_one = build_std_header(ws::POPUP | DS_FIXEDSYS, 1, "StdFixedOne");
            push_std_item(&mut std_one, 501, None, "ItemStd", 0, &[]);
            let parsed = parse_dialog_template(std_one.as_ptr(), Some(std_one.len()))
                .expect("1-control DS_FIXEDSYS standard template must parse control cleanly");
            assert_eq!(parsed.items.len(), 1);
            assert_eq!(parsed.items[0].id, 501);
            assert_eq!(String::from_utf16_lossy(&parsed.items[0].title), "ItemStd");

            // 3. Extended zero-control template with DS_FIXEDSYS alone.
            let ex_zero = build_ex_header(ws::POPUP | DS_FIXEDSYS, 0, "ExFixedZero");
            let parsed = parse_dialog_template(ex_zero.as_ptr(), Some(ex_zero.len()))
                .expect("zero-control DS_FIXEDSYS extended template must parse without font block");
            assert_eq!(String::from_utf16_lossy(&parsed.title), "ExFixedZero");
            assert!(parsed.items.is_empty());

            // 4. Extended 1-control template with DS_FIXEDSYS alone.
            let mut ex_one = build_ex_header(ws::POPUP | DS_FIXEDSYS, 1, "ExFixedOne");
            push_ex_item(&mut ex_one, 601, None, "ItemEx", 0, &[]);
            let parsed = parse_dialog_template(ex_one.as_ptr(), Some(ex_one.len()))
                .expect("1-control DS_FIXEDSYS extended template must parse control cleanly");
            assert_eq!(parsed.items.len(), 1);
            assert_eq!(parsed.items[0].id, 601);
            assert_eq!(String::from_utf16_lossy(&parsed.items[0].title), "ItemEx");

            // 5. Existing font-block cases: DS_SETFONT alone and DS_SHELLFONT (DS_SETFONT | DS_FIXEDSYS)
            //    both consume their respective font blocks in standard and extended templates.
            for style in [ws::POPUP | DS_SETFONT, ws::POPUP | DS_SHELLFONT] {
                let mut std_font = build_std_header(style, 1, "StdFont");
                push_u16(&mut std_font, 9);
                push_wide_z(&mut std_font, "Tahoma");
                push_std_item(&mut std_font, 502, None, "AfterStdFont", 0, &[]);
                let p_std = parse_dialog_template(std_font.as_ptr(), Some(std_font.len()))
                    .expect("standard template with DS_SETFONT/DS_SHELLFONT must parse font block");
                assert_eq!(p_std.items.len(), 1);
                assert_eq!(p_std.items[0].id, 502);
                assert_eq!(String::from_utf16_lossy(&p_std.items[0].title), "AfterStdFont");

                let mut ex_font = build_ex_header(style, 1, "ExFont");
                push_u16(&mut ex_font, 8);
                push_u16(&mut ex_font, 400);
                ex_font.push(0);
                ex_font.push(1);
                push_wide_z(&mut ex_font, "MS Shell Dlg 2");
                push_ex_item(&mut ex_font, 602, None, "AfterExFont", 0, &[]);
                let p_ex = parse_dialog_template(ex_font.as_ptr(), Some(ex_font.len()))
                    .expect("extended template with DS_SETFONT/DS_SHELLFONT must parse font block");
                assert_eq!(p_ex.items.len(), 1);
                assert_eq!(p_ex.items[0].id, 602);
                assert_eq!(String::from_utf16_lossy(&p_ex.items[0].title), "AfterExFont");
            }
        }
    }

    static CREATE_PARAM_PTR: AtomicUsize = AtomicUsize::new(0);
    static CREATE_PARAM_FIRST4: AtomicUsize = AtomicUsize::new(0);

    unsafe extern "win64" fn create_param_capture_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        if msg == wm::CREATE && lparam != 0 {
            let cs = &*(lparam as *const window::CREATESTRUCT);
            CREATE_PARAM_PTR.store(cs.create_params, Ordering::SeqCst);
            if cs.create_params != 0 {
                let v = (cs.create_params as *const u32).read_unaligned();
                CREATE_PARAM_FIRST4.store(v as usize, Ordering::SeqCst);
            }
        }
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }

    #[test]
    fn dialog_template_creation_data_standard_vs_extended_and_wm_create_param() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        assert!(!crate::macos::active(), "unit tests must stay headless");
        std::thread::spawn(|| unsafe {
            let proc_handle = super::super::register_typed_host_wndproc(create_param_capture_proc);
            let cls_name = "Aw2DlgCreateParamCtrl";
            let cls_w = crate::strings::to_wide_z(cls_name);
            let wc = class::WNDCLASS {
                style: 0,
                wndproc: proc_handle,
                cls_extra: 0,
                wnd_extra: 0,
                instance: 0,
                icon: 0,
                cursor: 0,
                background: 0,
                menu_name: 0,
                class_name: cls_w.as_ptr() as usize,
            };
            let _ = class::RegisterClassW(&wc);

            // 1. Standard DLGITEMTEMPLATE: size WORD includes itself (size=6 => 2-byte size + 4 payload bytes).
            let std_payload: [u8; 4] = [0x11, 0x22, 0x33, 0x44];
            let mut std_two = build_std_header(ws::POPUP, 2, "StdExtra");
            push_std_item(&mut std_two, 301, Some(cls_name), "First", 6, &std_payload);
            push_std_item(&mut std_two, 302, None, "AfterStd4", 0, &[]);
            let p_std = parse_dialog_template(std_two.as_ptr(), Some(std_two.len()))
                .expect("standard item with size=6 (4 payload bytes) + following control must parse");
            assert_eq!(p_std.items.len(), 2);
            assert_eq!(p_std.items[0].id, 301);
            assert_ne!(p_std.items[0].extra_ptr, 0);
            assert_eq!(
                std::slice::from_raw_parts(p_std.items[0].extra_ptr as *const u8, 4),
                &std_payload
            );
            assert_eq!(p_std.items[1].id, 302);
            assert_eq!(String::from_utf16_lossy(&p_std.items[1].title), "AfterStd4");

            // End-to-end headless creation verifies WM_CREATE lpCreateParams pointer and second control.
            CREATE_PARAM_PTR.store(0, Ordering::SeqCst);
            CREATE_PARAM_FIRST4.store(0, Ordering::SeqCst);
            let hdlg_std = CreateDialogIndirectParamW(
                std::ptr::null_mut(),
                std_two.as_ptr(),
                std::ptr::null_mut(),
                0,
                0,
            );
            assert!(!hdlg_std.is_null());
            assert_eq!(CREATE_PARAM_PTR.load(Ordering::SeqCst), p_std.items[0].extra_ptr);
            assert_eq!(
                CREATE_PARAM_FIRST4.load(Ordering::SeqCst),
                u32::from_le_bytes(std_payload) as usize
            );
            let mut txt = [0u16; 32];
            let n = GetDlgItemTextW(hdlg_std, 302, txt.as_mut_ptr(), 32);
            assert_eq!(String::from_utf16_lossy(&txt[..n as usize]), "AfterStd4");
            assert_eq!(DestroyWindow(hdlg_std), TRUE);

            // 2. Standard DLGITEMTEMPLATE odd size=5 (3 payload bytes) + DWORD-aligned following control.
            let std_odd_payload: [u8; 3] = [0xAA, 0xBB, 0xCC];
            let mut std_odd = build_std_header(ws::POPUP, 2, "StdOdd");
            push_std_item(&mut std_odd, 303, None, "Odd", 5, &std_odd_payload);
            push_std_item(&mut std_odd, 304, None, "AfterStdOdd", 0, &[]);
            let p_odd = parse_dialog_template(std_odd.as_ptr(), Some(std_odd.len()))
                .expect("standard odd creation-data size=5 + DWORD-aligned successor must parse");
            assert_eq!(
                std::slice::from_raw_parts(p_odd.items[0].extra_ptr as *const u8, 3),
                &std_odd_payload
            );
            assert_eq!(p_odd.items[1].id, 304);
            assert_eq!(String::from_utf16_lossy(&p_odd.items[1].title), "AfterStdOdd");

            // 3. Standard DLGITEMTEMPLATE size=2 (size WORD only, 0 trailing payload bytes) + following control.
            let mut std_two_only = build_std_header(ws::POPUP, 2, "StdTwo");
            push_std_item(&mut std_two_only, 305, None, "TwoOnly", 2, &[]);
            push_std_item(&mut std_two_only, 306, None, "AfterStd2", 0, &[]);
            let p_two = parse_dialog_template(std_two_only.as_ptr(), Some(std_two_only.len()))
                .expect("standard creation-data size=2 must consume only the size WORD");
            assert_eq!(p_two.items[0].extra_ptr, 0);
            assert_eq!(p_two.items[1].id, 306);
            assert_eq!(String::from_utf16_lossy(&p_two.items[1].title), "AfterStd2");

            // 4. Malformed standard size=1 must be rejected.
            let mut std_bad_one = build_std_header(ws::POPUP, 1, "StdBad1");
            push_std_item(&mut std_bad_one, 307, None, "Bad1", 1, &[]);
            assert!(
                parse_dialog_template(std_bad_one.as_ptr(), Some(std_bad_one.len())).is_err(),
                "standard DLGITEMTEMPLATE creation-data size=1 is malformed and must fail"
            );

            // 5. Truncated standard payload (size=6 claims 4 payload bytes, only 3 present) must be rejected.
            let mut std_trunc = build_std_header(ws::POPUP, 1, "StdTrunc");
            push_std_item(&mut std_trunc, 308, None, "Trunc", 6, &[0x01, 0x02, 0x03]);
            assert!(
                parse_dialog_template(std_trunc.as_ptr(), Some(std_trunc.len())).is_err(),
                "truncated standard creation-data payload must fail"
            );

            // 6. Extended DLGITEMTEMPLATEEX: extraCount excludes the size WORD (extraCount=4 => 4 bytes follow).
            let ex_payload: [u8; 4] = [0x55, 0x66, 0x77, 0x88];
            let mut ex_two = build_ex_header(ws::POPUP, 2, "ExExtra");
            push_ex_item(&mut ex_two, 401, Some(cls_name), "FirstEx", 4, &ex_payload);
            push_ex_item(&mut ex_two, 402, None, "AfterEx4", 0, &[]);
            let p_ex = parse_dialog_template(ex_two.as_ptr(), Some(ex_two.len()))
                .expect("extended item with extraCount=4 + following control must parse");
            assert_eq!(p_ex.items.len(), 2);
            assert_eq!(p_ex.items[0].id, 401);
            assert_eq!(
                std::slice::from_raw_parts(p_ex.items[0].extra_ptr as *const u8, 4),
                &ex_payload
            );
            assert_eq!(p_ex.items[1].id, 402);
            assert_eq!(String::from_utf16_lossy(&p_ex.items[1].title), "AfterEx4");

            CREATE_PARAM_PTR.store(0, Ordering::SeqCst);
            CREATE_PARAM_FIRST4.store(0, Ordering::SeqCst);
            let hdlg_ex = CreateDialogIndirectParamW(
                std::ptr::null_mut(),
                ex_two.as_ptr(),
                std::ptr::null_mut(),
                0,
                0,
            );
            assert!(!hdlg_ex.is_null());
            assert_eq!(CREATE_PARAM_PTR.load(Ordering::SeqCst), p_ex.items[0].extra_ptr);
            assert_eq!(
                CREATE_PARAM_FIRST4.load(Ordering::SeqCst),
                u32::from_le_bytes(ex_payload) as usize
            );
            let n_ex = GetDlgItemTextW(hdlg_ex, 402, txt.as_mut_ptr(), 32);
            assert_eq!(String::from_utf16_lossy(&txt[..n_ex as usize]), "AfterEx4");
            assert_eq!(DestroyWindow(hdlg_ex), TRUE);

            // 7. Extended DLGITEMTEMPLATEEX extraCount=1 and extraCount=2 (valid!) + following controls.
            let mut ex_one = build_ex_header(ws::POPUP, 2, "ExOne");
            push_ex_item(&mut ex_one, 403, None, "OneByte", 1, &[0x99]);
            push_ex_item(&mut ex_one, 404, None, "AfterEx1", 0, &[]);
            let p_ex1 = parse_dialog_template(ex_one.as_ptr(), Some(ex_one.len()))
                .expect("extended extraCount=1 + DWORD-aligned successor must parse");
            assert_eq!(
                std::slice::from_raw_parts(p_ex1.items[0].extra_ptr as *const u8, 1),
                &[0x99]
            );
            assert_eq!(p_ex1.items[1].id, 404);
            assert_eq!(String::from_utf16_lossy(&p_ex1.items[1].title), "AfterEx1");

            let mut ex_two_b = build_ex_header(ws::POPUP, 2, "ExTwoBytes");
            push_ex_item(&mut ex_two_b, 405, None, "TwoBytes", 2, &[0xDE, 0xAD]);
            push_ex_item(&mut ex_two_b, 406, None, "AfterEx2", 0, &[]);
            let p_ex2 = parse_dialog_template(ex_two_b.as_ptr(), Some(ex_two_b.len()))
                .expect("extended extraCount=2 + following control must parse");
            assert_eq!(
                std::slice::from_raw_parts(p_ex2.items[0].extra_ptr as *const u8, 2),
                &[0xDE, 0xAD]
            );
            assert_eq!(p_ex2.items[1].id, 406);
            assert_eq!(String::from_utf16_lossy(&p_ex2.items[1].title), "AfterEx2");

            // 8. Truncated extended payload (extraCount=4, only 3 bytes present) must be rejected.
            let mut ex_trunc = build_ex_header(ws::POPUP, 1, "ExTrunc");
            push_ex_item(&mut ex_trunc, 407, None, "TruncEx", 4, &[0x01, 0x02, 0x03]);
            assert!(
                parse_dialog_template(ex_trunc.as_ptr(), Some(ex_trunc.len())).is_err(),
                "truncated extended creation-data payload must fail"
            );
        })
        .join()
        .unwrap();
    }
}
