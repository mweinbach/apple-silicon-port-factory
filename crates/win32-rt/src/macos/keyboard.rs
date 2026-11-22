//! Active macOS keyboard layout (`'uchr'`) cache and `UCKeyTranslate` bridge
//! for `user32::ToUnicode` / `ToUnicodeEx`.
//!
//! # Threading
//!
//! Carbon's `TISCopyCurrentKeyboardLayoutInputSource` asserts the main thread
//! on modern macOS, whereas `UCKeyTranslate` is a pure read over the raw
//! `'uchr'` byte buffer. Therefore:
//! * On the main thread (at `appkit::init` and whenever
//!   `kTISNotifySelectedKeyboardInputSourceChanged` fires), we copy the
//!   `'uchr'` bytes and `LMGetKbdType()` into a `RwLock` cache.
//! * Win32 threads call `UCKeyTranslate` against the cached byte copy with
//!   zero HIToolbox/TSM calls and zero CoreFoundation retain/release traffic.
//! * `Carbon.framework` is loaded dynamically via `dlopen` so no extra link
//!   flags are needed.

use std::sync::RwLock;

/// Cached copy of the active macOS `'uchr'` layout table.
#[derive(Clone, Debug)]
pub struct KeyboardLayout {
    pub uchr: Vec<u8>,
    pub kbd_type: u32,
}

/// Outcome of translating one keystroke via macOS `UCKeyTranslate`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TranslateOutcome {
    /// One or more UTF-16 code units produced.
    Chars(Vec<u16>),
    /// A dead key was entered; payload holds the spacing dead-key glyph(s)
    /// (for filling `ToUnicode`'s buffer when returning `-1`).
    DeadKey(Vec<u16>),
    /// The key produced no character in the active layout.
    None,
    /// Carbon / `'uchr'` layout is unavailable; caller should fall back to
    /// the static US keymap table.
    Unavailable,
}

static LAYOUT: RwLock<Option<KeyboardLayout>> = RwLock::new(None);

#[cfg(test)]
static TEST_OVERRIDE: RwLock<Option<Option<KeyboardLayout>>> = RwLock::new(None);

#[cfg(target_os = "macos")]
mod carbon {
    use super::{KeyboardLayout, TranslateOutcome};
    use std::ffi::{c_char, c_void};
    use std::sync::OnceLock;

    const K_UC_KEY_ACTION_DOWN: u16 = 0;
    const K_UC_KEY_TRANSLATE_NO_DEAD_KEYS_MASK: u32 = 1;
    const K_VK_SPACE: u16 = 0x31;

    type TisCopyCurrentFn = unsafe extern "C" fn() -> *mut c_void;
    type TisGetPropFn = unsafe extern "C" fn(*mut c_void, *const c_void) -> *const c_void;
    type LmGetKbdTypeFn = unsafe extern "C" fn() -> u8;
    type UcKeyTranslateFn = unsafe extern "C" fn(
        *const u8,
        u16,
        u16,
        u32,
        u32,
        u32,
        *mut u32,
        usize,
        *mut usize,
        *mut u16,
    ) -> i32;
    type CfDataGetBytePtrFn = unsafe extern "C" fn(*const c_void) -> *const u8;
    type CfDataGetLengthFn = unsafe extern "C" fn(*const c_void) -> isize;
    type CfReleaseFn = unsafe extern "C" fn(*const c_void);
    type CfNotifCenterGetDistFn = unsafe extern "C" fn() -> *mut c_void;
    type CfNotifAddObserverFn = unsafe extern "C" fn(
        *mut c_void,
        *const c_void,
        unsafe extern "C" fn(*mut c_void, *mut c_void, *const c_void, *const c_void, *const c_void),
        *const c_void,
        *const c_void,
        isize,
    );

    pub(super) struct Api {
        tis_copy_current: TisCopyCurrentFn,
        tis_copy_ascii: Option<TisCopyCurrentFn>,
        tis_get_prop: TisGetPropFn,
        prop_uchr_key: *const c_void,
        notify_changed_key: Option<*const c_void>,
        lm_get_kbd_type: LmGetKbdTypeFn,
        uc_key_translate: UcKeyTranslateFn,
        cf_data_get_byte_ptr: CfDataGetBytePtrFn,
        cf_data_get_length: CfDataGetLengthFn,
        cf_release: CfReleaseFn,
        cf_dist_center: Option<CfNotifCenterGetDistFn>,
        cf_add_observer: Option<CfNotifAddObserverFn>,
    }

    // Function pointers and immutable CoreFoundation global string constants.
    unsafe impl Send for Api {}
    unsafe impl Sync for Api {}

    unsafe fn sym<T>(handle: *mut c_void, name: &[u8]) -> Option<T> {
        let p = libc::dlsym(handle, name.as_ptr() as *const c_char);
        if p.is_null() {
            None
        } else {
            Some(std::mem::transmute_copy(&p))
        }
    }

    unsafe fn sym_ptr_deref(handle: *mut c_void, name: &[u8]) -> Option<*const c_void> {
        let p = libc::dlsym(handle, name.as_ptr() as *const c_char);
        if p.is_null() {
            return None;
        }
        let val = *(p as *const *const c_void);
        (!val.is_null()).then_some(val)
    }

    pub(super) fn api() -> Option<&'static Api> {
        static API: OnceLock<Option<Api>> = OnceLock::new();
        API.get_or_init(|| unsafe {
            let h = libc::dlopen(
                c"/System/Library/Frameworks/Carbon.framework/Carbon".as_ptr(),
                libc::RTLD_LAZY | libc::RTLD_LOCAL,
            );
            if h.is_null() {
                log::warn!("keyboard: failed to dlopen Carbon.framework; falling back to US layout");
                return None;
            }
            Some(Api {
                tis_copy_current: sym(h, b"TISCopyCurrentKeyboardLayoutInputSource\0")?,
                tis_copy_ascii: sym(h, b"TISCopyCurrentASCIICapableKeyboardLayoutInputSource\0"),
                tis_get_prop: sym(h, b"TISGetInputSourceProperty\0")?,
                prop_uchr_key: sym_ptr_deref(h, b"kTISPropertyUnicodeKeyLayoutData\0")?,
                notify_changed_key: sym_ptr_deref(h, b"kTISNotifySelectedKeyboardInputSourceChanged\0"),
                lm_get_kbd_type: sym(h, b"LMGetKbdType\0")?,
                uc_key_translate: sym(h, b"UCKeyTranslate\0")?,
                cf_data_get_byte_ptr: sym(h, b"CFDataGetBytePtr\0")?,
                cf_data_get_length: sym(h, b"CFDataGetLength\0")?,
                cf_release: sym(h, b"CFRelease\0")?,
                cf_dist_center: sym(h, b"CFNotificationCenterGetDistributedCenter\0"),
                cf_add_observer: sym(h, b"CFNotificationCenterAddObserver\0"),
            })
        })
        .as_ref()
    }

    /// Read the active keyboard layout's `'uchr'` data. Main thread only!
    pub(super) fn fetch_current_layout() -> Option<KeyboardLayout> {
        let a = api()?;
        unsafe {
            let mut src = (a.tis_copy_current)();
            if src.is_null() {
                return None;
            }
            let mut data = (a.tis_get_prop)(src, a.prop_uchr_key);
            if data.is_null() {
                // CJK / non-Unicode IMEs return NULL for uchr; fall back to ASCII-capable layout.
                (a.cf_release)(src);
                src = a.tis_copy_ascii.map(|f| f()).unwrap_or(std::ptr::null_mut());
                if src.is_null() {
                    return None;
                }
                data = (a.tis_get_prop)(src, a.prop_uchr_key);
            }
            let layout = if !data.is_null() {
                let len = (a.cf_data_get_length)(data);
                let ptr = (a.cf_data_get_byte_ptr)(data);
                if len > 0 && !ptr.is_null() {
                    let bytes = std::slice::from_raw_parts(ptr, len as usize).to_vec();
                    let kbd_type = (a.lm_get_kbd_type)() as u32;
                    Some(KeyboardLayout { uchr: bytes, kbd_type })
                } else {
                    None
                }
            } else {
                None
            };
            (a.cf_release)(src);
            layout
        }
    }

    unsafe extern "C" fn on_layout_changed(
        _center: *mut c_void,
        _observer: *mut c_void,
        _name: *const c_void,
        _obj: *const c_void,
        _info: *const c_void,
    ) {
        super::refresh();
    }

    pub(super) fn register_observer() {
        static REGISTERED: OnceLock<()> = OnceLock::new();
        REGISTERED.get_or_init(|| {
            let Some(a) = api() else { return };
            let (Some(get_center), Some(add_obs), Some(name)) =
                (a.cf_dist_center, a.cf_add_observer, a.notify_changed_key)
            else {
                return;
            };
            unsafe {
                let center = get_center();
                if !center.is_null() {
                    // CFNotificationSuspensionBehaviorDeliverImmediately = 4
                    add_obs(center, std::ptr::null(), on_layout_changed, name, std::ptr::null(), 4);
                }
            }
        });
    }

    pub(super) fn translate_with_uchr(
        uchr: &[u8],
        kbd_type: u32,
        mac_keycode: u16,
        shift: bool,
        caps: bool,
        option: bool,
        ctrl: bool,
        preserve_state: bool,
        dead_key_state: &mut u32,
    ) -> TranslateOutcome {
        let Some(a) = api() else {
            return TranslateOutcome::Unavailable;
        };
        if uchr.len() < 40 {
            return TranslateOutcome::Unavailable;
        }
        // Carbon modifierKeyState = (EventModifiers >> 8) & 0xFF:
        // bit 1 = shiftKey, bit 2 = alphaLock, bit 3 = optionKey, bit 4 = controlKey.
        let mut mods = 0u32;
        if shift {
            mods |= 1 << 1;
        }
        if caps {
            mods |= 1 << 2;
        }
        if option {
            mods |= 1 << 3;
        }
        if ctrl {
            mods |= 1 << 4;
        }

        let mut local_dead = *dead_key_state;
        let mut out = [0u16; 8];
        let mut actual_len = 0usize;
        let st = unsafe {
            (a.uc_key_translate)(
                uchr.as_ptr(),
                mac_keycode,
                K_UC_KEY_ACTION_DOWN,
                mods,
                kbd_type,
                0,
                &mut local_dead,
                out.len(),
                &mut actual_len,
                out.as_mut_ptr(),
            )
        };
        if st != 0 {
            return TranslateOutcome::Unavailable;
        }

        let active_dead = (local_dead & 0xFFFF) != 0;
        if !preserve_state {
            *dead_key_state = if active_dead { local_dead } else { 0 };
        }

        if actual_len == 0 && active_dead {
            // Query the spacing dead-key character (e.g. '´', '^', '`') by
            // translating Space against a copy of the dead-key state.
            let mut probe_dead = local_dead;
            let mut dead_out = [0u16; 8];
            let mut dead_len = 0usize;
            let _ = unsafe {
                (a.uc_key_translate)(
                    uchr.as_ptr(),
                    K_VK_SPACE,
                    K_UC_KEY_ACTION_DOWN,
                    0,
                    kbd_type,
                    K_UC_KEY_TRANSLATE_NO_DEAD_KEYS_MASK,
                    &mut probe_dead,
                    dead_out.len(),
                    &mut dead_len,
                    dead_out.as_mut_ptr(),
                )
            };
            let n = dead_len.min(dead_out.len());
            return TranslateOutcome::DeadKey(dead_out[..n].to_vec());
        }

        if actual_len > 0 {
            let n = actual_len.min(out.len());
            TranslateOutcome::Chars(out[..n].to_vec())
        } else {
            TranslateOutcome::None
        }
    }
}

/// Re-read the active macOS keyboard layout into the cache. Main thread only.
pub fn refresh() {
    #[cfg(target_os = "macos")]
    {
        if let Some(layout) = carbon::fetch_current_layout() {
            log::debug!(
                "keyboard: cached macOS uchr layout ({} bytes, kbd_type={})",
                layout.uchr.len(),
                layout.kbd_type
            );
            *LAYOUT.write().unwrap() = Some(layout);
        }
    }
}

/// Load the initial layout and subscribe to layout-change notifications. Main thread only.
#[cfg(target_os = "macos")]
pub(super) fn init_main_thread() {
    refresh();
    carbon::register_observer();
}

fn ensure_loaded() {
    if LAYOUT.read().unwrap().is_some() {
        return;
    }
    if super::is_main_thread() {
        refresh();
    } else if super::active() {
        let _ = super::run_sync(refresh);
    } else if cfg!(test) {
        refresh();
    }
}

/// Translate `mac_keycode` (`kVK_*`) using the active macOS keyboard layout.
///
/// Returns `TranslateOutcome::Unavailable` when headless on a worker thread or
/// if Carbon cannot be loaded, signalling the caller to fall back to the US
/// keymap table.
pub fn translate_key(
    mac_keycode: u16,
    shift: bool,
    caps: bool,
    option: bool,
    ctrl: bool,
    preserve_state: bool,
    dead_key_state: &mut u32,
) -> TranslateOutcome {
    #[cfg(test)]
    if let Some(ov) = TEST_OVERRIDE.read().unwrap().as_ref() {
        return match ov {
            Some(l) => translate_with_uchr(
                &l.uchr,
                l.kbd_type,
                mac_keycode,
                shift,
                caps,
                option,
                ctrl,
                preserve_state,
                dead_key_state,
            ),
            None => TranslateOutcome::Unavailable,
        };
    }

    ensure_loaded();
    let guard = LAYOUT.read().unwrap();
    let Some(layout) = guard.as_ref() else {
        return TranslateOutcome::Unavailable;
    };
    translate_with_uchr(
        &layout.uchr,
        layout.kbd_type,
        mac_keycode,
        shift,
        caps,
        option,
        ctrl,
        preserve_state,
        dead_key_state,
    )
}

/// Translate `mac_keycode` against an explicit `'uchr'` byte buffer.
pub fn translate_with_uchr(
    uchr: &[u8],
    kbd_type: u32,
    mac_keycode: u16,
    shift: bool,
    caps: bool,
    option: bool,
    ctrl: bool,
    preserve_state: bool,
    dead_key_state: &mut u32,
) -> TranslateOutcome {
    #[cfg(target_os = "macos")]
    {
        carbon::translate_with_uchr(
            uchr,
            kbd_type,
            mac_keycode,
            shift,
            caps,
            option,
            ctrl,
            preserve_state,
            dead_key_state,
        )
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (uchr, kbd_type, mac_keycode, shift, caps, option, ctrl, preserve_state, dead_key_state);
        TranslateOutcome::Unavailable
    }
}

#[cfg(test)]
pub(crate) fn with_test_layout<R>(layout: Option<KeyboardLayout>, f: impl FnOnce() -> R) -> R {
    *TEST_OVERRIDE.write().unwrap() = Some(layout);
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            *TEST_OVERRIDE.write().unwrap() = None;
        }
    }
    let _reset = Reset;
    f()
}

#[cfg(test)]
pub(crate) fn build_test_uchr(normal: &[(u16, u16)], shifted: &[(u16, u16)], option: &[(u16, u16)]) -> KeyboardLayout {
    // Construct a minimal, valid Apple 'uchr' binary resource:
    //   0..12   UCKeyboardLayout header
    //   12..40  UCKeyboardTypeHeader (default entry: first=0, last=0)
    //   40..64  UCKeyModifiersToTableNum (16 entries: 0->0, 2->1, 4->1, 6->0, 8->2)
    //   64..84  UCKeyToCharTableIndex (3 tables of 128 u16s)
    //   84..340   table 0 (unshifted)
    //   340..596  table 1 (shifted / caps)
    //   596..852  table 2 (option)
    let mut b = Vec::with_capacity(852);
    let push_u16 = |v: &mut Vec<u8>, x: u16| v.extend_from_slice(&x.to_ne_bytes());
    let push_u32 = |v: &mut Vec<u8>, x: u32| v.extend_from_slice(&x.to_ne_bytes());

    // UCKeyboardLayout
    push_u16(&mut b, 0x1002); // kUCKeyLayoutHeaderFormat
    push_u16(&mut b, 0x0100); // version 1.0
    push_u32(&mut b, 0);      // featureInfoOffset
    push_u32(&mut b, 1);      // keyboardTypeCount

    // UCKeyboardTypeHeader @ 12
    push_u32(&mut b, 0);  // keyboardTypeFirst
    push_u32(&mut b, 0);  // keyboardTypeLast
    push_u32(&mut b, 40); // keyModifiersToTableNumOffset
    push_u32(&mut b, 64); // keyToCharTableIndexOffset
    push_u32(&mut b, 0);  // keyStateRecordsIndexOffset
    push_u32(&mut b, 0);  // keyStateTerminatorsOffset
    push_u32(&mut b, 0);  // keySequenceDataIndexOffset
    assert_eq!(b.len(), 40);

    // UCKeyModifiersToTableNum @ 40
    push_u16(&mut b, 0x3001); // kUCKeyModifiersToTableNumFormat
    push_u16(&mut b, 0);      // defaultTableNum
    push_u32(&mut b, 16);     // modifiersCount
    let mut mod_map = [0u8; 16];
    mod_map[2] = 1; // Shift -> table 1
    mod_map[4] = 1; // Caps -> table 1
    mod_map[6] = 0; // Shift+Caps -> table 0
    mod_map[8] = 2; // Option -> table 2
    b.extend_from_slice(&mod_map);
    assert_eq!(b.len(), 64);

    // UCKeyToCharTableIndex @ 64
    push_u16(&mut b, 0x4001); // kUCKeyToCharTableIndexFormat
    push_u16(&mut b, 128);    // keyToCharTableSize
    push_u32(&mut b, 3);      // keyToCharTableCount
    push_u32(&mut b, 84);     // table 0
    push_u32(&mut b, 340);    // table 1
    push_u32(&mut b, 596);    // table 2
    assert_eq!(b.len(), 84);

    for entries in [normal, shifted, option] {
        let mut tbl = [0xFFFFu16; 128]; // 0xFFFF = no char
        for &(kc, ch) in entries {
            if (kc as usize) < 128 {
                tbl[kc as usize] = ch;
            }
        }
        for u in tbl {
            push_u16(&mut b, u);
        }
    }
    assert_eq!(b.len(), 852);
    KeyboardLayout { uchr: b, kbd_type: 40 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synthetic_non_us_uchr_translation() {
        // Simulated AZERTY + German umlaut + Euro sign via real Apple UCKeyTranslate.
        let azerty = build_test_uchr(
            &[
                (0x0C, b'a' as u16),  // physical Q -> 'a'
                (0x00, b'q' as u16),  // physical A -> 'q'
                (0x21, 0x00FC),       // physical [ -> 'ü'
                (0x12, b'&' as u16),  // physical 1 -> '&'
            ],
            &[
                (0x0C, b'A' as u16),  // Shift+Q -> 'A'
                (0x00, b'Q' as u16),  // Shift+A -> 'Q'
                (0x21, 0x00DC),       // Shift+[ -> 'Ü'
                (0x12, b'1' as u16),  // Shift+1 -> '1'
            ],
            &[
                (0x0E, 0x20AC),       // Option+E -> '€'
            ],
        );
        let mut dead = 0u32;
        assert_eq!(
            translate_with_uchr(&azerty.uchr, azerty.kbd_type, 0x0C, false, false, false, false, false, &mut dead),
            TranslateOutcome::Chars(vec![b'a' as u16])
        );
        assert_eq!(
            translate_with_uchr(&azerty.uchr, azerty.kbd_type, 0x0C, true, false, false, false, false, &mut dead),
            TranslateOutcome::Chars(vec![b'A' as u16])
        );
        assert_eq!(
            translate_with_uchr(&azerty.uchr, azerty.kbd_type, 0x21, false, false, false, false, false, &mut dead),
            TranslateOutcome::Chars(vec![0x00FC])
        );
        assert_eq!(
            translate_with_uchr(&azerty.uchr, azerty.kbd_type, 0x0E, false, false, true, false, false, &mut dead),
            TranslateOutcome::Chars(vec![0x20AC])
        );
        // Unmapped key returns None cleanly.
        assert_eq!(
            translate_with_uchr(&azerty.uchr, azerty.kbd_type, 0x0F, false, false, false, false, false, &mut dead),
            TranslateOutcome::None
        );
    }

    #[test]
    fn live_macos_layout_loads_and_translates() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let mut dead = 0u32;
        // Space (0x31) produces ' ' on every macOS keyboard layout.
        let r = translate_key(0x31, false, false, false, false, false, &mut dead);
        assert_eq!(r, TranslateOutcome::Chars(vec![b' ' as u16]));
    }
}
