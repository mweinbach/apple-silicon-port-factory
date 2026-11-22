//! `DefWindowProc`: the default handling the game falls back to.

use super::message::send_message;
use super::window::{self, WINDOWPOS};
use super::*;
use crate::types::*;

pub const SC_SIZE: usize = 0xF000;
pub const SC_MOVE: usize = 0xF010;
pub const SC_MINIMIZE: usize = 0xF020;
pub const SC_MAXIMIZE: usize = 0xF030;
pub const SC_CLOSE: usize = 0xF060;
pub const SC_KEYMENU: usize = 0xF100;
pub const SC_RESTORE: usize = 0xF120;

pub const HTCLIENT: isize = 1;
pub const HTCAPTION: isize = 2;
pub const HTNOWHERE: isize = 0;

const SWP_NOSIZE: u32 = window::SWP_NOSIZE;
const SWP_NOMOVE: u32 = window::SWP_NOMOVE;
const SWP_STATECHANGED: u32 = window::SWP_STATECHANGED;

pub unsafe fn def_window_proc(hwnd: usize, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        wm::NCCREATE => {
            let cs = lparam as *const window::CREATESTRUCT;
            if !cs.is_null() && (*cs).name != 0 && !is_int_resource((*cs).name as *const u16) {
                window::set_internal_text(hwnd, crate::strings::wide_slice((*cs).name as *const u16).to_vec());
            }
            1
        }
        wm::NCCALCSIZE => {
            // lParam: RECT* (wParam FALSE) or NCCALCSIZE_PARAMS* (rgrc[0] first).
            if lparam != 0 {
                if let Some(w) = window::get(hwnd) {
                    let (style, exstyle) = {
                        let d = w.data();
                        (d.style, d.exstyle)
                    };
                    let r = lparam as *mut RECT;
                    let cur = r.read_unaligned();
                    let s = crate::macos::display::scale_for_rect(&cur);
                    r.write_unaligned(window::client_from_window_for_scale(&cur, style, exstyle, s));
                }
            }
            0
        }
        wm::NCHITTEST => {
            let x = (lparam & 0xFFFF) as i16 as i32;
            let y = ((lparam >> 16) & 0xFFFF) as i16 as i32;
            let Some(r) = window::window_rect_screen(hwnd) else { return HTNOWHERE };
            if !r.contains(x, y) {
                return HTNOWHERE;
            }
            match (window::client_origin_screen(hwnd), window::client_size(hwnd)) {
                (Some(o), Some((w, h))) if RECT::new(o.x, o.y, o.x + w, o.y + h).contains(x, y) => HTCLIENT,
                _ => HTCAPTION,
            }
        }
        wm::NCACTIVATE => 1,
        wm::NCPAINT | wm::NCDESTROY | wm::DESTROY => 0,
        wm::CLOSE => {
            super::window::DestroyWindow(hwnd as HWND);
            0
        }
        wm::PAINT => {
            let mut ps = std::mem::zeroed::<window::PAINTSTRUCT>();
            window::BeginPaint(hwnd as HWND, &mut ps);
            window::EndPaint(hwnd as HWND, &ps);
            0
        }
        wm::ERASEBKGND => {
            // The class background brush would be painted here. Nothing is
            // drawn into the Metal layer; report it as erased if a brush exists.
            window::get(hwnd).map(|w| (w.class.background != 0) as LRESULT).unwrap_or(0)
        }
        wm::SETTEXT => {
            let text = if lparam == 0 { Vec::new() } else { crate::strings::wide_slice(lparam as *const u16).to_vec() };
            window::set_internal_text(hwnd, text);
            1
        }
        wm::GETTEXT => {
            let Some(t) = window::internal_text(hwnd) else { return 0 };
            if wparam == 0 || lparam == 0 {
                return 0;
            }
            let n = t.len().min(wparam - 1);
            std::ptr::copy_nonoverlapping(t.as_ptr(), lparam as *mut u16, n);
            *(lparam as *mut u16).add(n) = 0;
            n as LRESULT
        }
        wm::GETTEXTLENGTH => window::internal_text(hwnd).map(|t| t.len() as LRESULT).unwrap_or(0),
        wm::WINDOWPOSCHANGED => {
            let wp = &*(lparam as *const WINDOWPOS);
            let Some(w) = window::get(hwnd) else { return 0 };
            let (c, minimized, maximized) = {
                let d = w.data();
                (d.client, d.minimized, d.maximized)
            };
            if wp.flags & SWP_NOMOVE == 0 || wp.flags & SWP_STATECHANGED != 0 {
                send_message(hwnd, wm::MOVE, 0, make_long(c.left, c.top));
            }
            if wp.flags & SWP_NOSIZE == 0 || wp.flags & (SWP_STATECHANGED | window::SWP_FRAMECHANGED) != 0 {
                let (kind, cw, ch) = if minimized {
                    (window::SIZE_MINIMIZED, 0, 0)
                } else if maximized {
                    (window::SIZE_MAXIMIZED, c.width(), c.height())
                } else {
                    (window::SIZE_RESTORED, c.width(), c.height())
                };
                send_message(hwnd, wm::SIZE, kind, make_long(cw, ch));
            }
            0
        }
        wm::WINDOWPOSCHANGING | wm::GETMINMAXINFO | wm::SHOWWINDOW | wm::MOVE | wm::SIZE => 0,
        wm::ACTIVATE => {
            // WA_ACTIVE / WA_CLICKACTIVE give the window the keyboard focus.
            if wparam & 0xFFFF != 0 && (wparam >> 16) & 0xFFFF == 0 {
                super::input::set_focus(hwnd);
            }
            0
        }
        wm::MOUSEACTIVATE => 1, // MA_ACTIVATE
        wm::SETCURSOR => {
            if (lparam & 0xFFFF) as i16 as isize == HTCLIENT {
                // Only a non-NULL class cursor is applied; with NULL the
                // application manages the cursor itself (as on Windows).
                if let Some(w) = window::get(hwnd) {
                    let c = *w.class.cursor.lock().unwrap();
                    if c != 0 {
                        super::cursor::set_cursor(c);
                        return 1;
                    }
                }
            }
            0
        }
        wm::SYSCOMMAND => {
            match wparam & 0xFFF0 {
                SC_CLOSE => {
                    send_message(hwnd, wm::CLOSE, 0, 0);
                }
                SC_MINIMIZE => {
                    window::ShowWindow(hwnd as HWND, window::SW_MINIMIZE);
                }
                SC_MAXIMIZE => {
                    window::ShowWindow(hwnd as HWND, window::SW_MAXIMIZE);
                }
                SC_RESTORE => {
                    window::ShowWindow(hwnd as HWND, window::SW_RESTORE);
                }
                SC_KEYMENU => {} // no menu bar
                other => stub!("DefWindowProc(WM_SYSCOMMAND {other:#x}) ignored"),
            }
            0
        }
        wm::SYSKEYDOWN => {
            // Alt+F4 closes the window.
            if wparam == 0x73 && (lparam >> 29) & 1 != 0 {
                send_message(hwnd, wm::SYSCOMMAND, SC_CLOSE, 0);
            }
            0
        }
        wm::KEYDOWN | wm::KEYUP | wm::SYSKEYUP | wm::CHAR | wm::SYSCHAR | wm::DEADCHAR => 0,
        wm::GETICON | wm::SETICON => 0,
        wm::INPUT => 0,
        _ => 0,
    }
}

pub unsafe extern "win64" fn DefWindowProcW(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    def_window_proc(hwnd as usize, msg, wparam, lparam)
}

pub unsafe extern "win64" fn DefWindowProcA(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        wm::SETTEXT if lparam != 0 => {
            let w = crate::strings::to_wide_z(&crate::strings::from_ansi(lparam as *const u8));
            def_window_proc(hwnd as usize, msg, wparam, w.as_ptr() as LPARAM)
        }
        _ => def_window_proc(hwnd as usize, msg, wparam, lparam),
    }
}
