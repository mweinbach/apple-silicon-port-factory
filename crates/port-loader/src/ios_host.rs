//! C-ABI embedding host for iOS (`UIApplicationMain` / `CAMetalLayer`).
//!
//! On iOS, the Swift/Objective-C host app owns the process main thread through
//! `UIApplicationMain` and embeds `libport_loader.a` (built via
//! `cargo build --release -p port-loader --lib --no-default-features --target aarch64-apple-ios`).
//!
//! Lifecycle:
//! 1. The host creates its fullscreen `CAMetalLayer`-backed `UIView` and calls
//!    [`port_ios_attach_surface`] with the layer pointer and physical pixel dimensions.
//! 2. (Optional, Milestone #2) Any statically linked Mach-O `.o` AOT modules register
//!    themselves via [`port_ios_register_static_aot`].
//! 3. The host calls [`port_ios_start_background`] with optional paths (or `NULL` to
//!    auto-discover `<App>.app/game.toml` and `<App>.app/Game`). The Windows runtime
//!    boots on a dedicated background thread so UIKit never blocks.
//! 4. On resize/rotation/touch/key/display-link ticks, the host calls
//!    [`port_ios_resize_surface`], [`port_ios_touch_event`], [`port_ios_key_event`],
//!    and [`port_ios_tick`].

use std::ffi::{c_char, c_void, CStr};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

/// Optional startup configuration passed from Swift/Objective-C to
/// [`port_ios_start_background`]. Any pointer may be `NULL` (or empty) to use
/// standard flat iOS bundle discovery (`<App>.app/game.toml`, `<App>.app/Game`,
/// `~/Documents/Translator`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PortIosConfig {
    /// Host path to the game directory (e.g. `<bundle>/Game`).
    pub game_dir: *const c_char,
    /// Executable filename inside `game_dir` (e.g. `"Game.exe"`).
    pub exe_name: *const c_char,
    /// Optional path to `game.toml`.
    pub manifest_path: *const c_char,
    /// Optional writable `C:\` root override (defaults to `~/Documents/Translator` on iOS).
    pub drive_c_dir: *const c_char,
}

static STARTED: AtomicBool = AtomicBool::new(false);

unsafe fn opt_cstr(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    let s = CStr::from_ptr(ptr).to_str().ok()?.trim();
    (!s.is_empty()).then(|| s.to_string())
}

/// Bind the host `UIView`'s `CAMetalLayer*` and physical drawable dimensions.
#[no_mangle]
pub unsafe extern "C" fn port_ios_attach_surface(
    metal_layer: *mut c_void,
    width_px: u32,
    height_px: u32,
    scale: f64,
) {
    win32_rt::platform::attach_host_surface(metal_layer, width_px, height_px, scale);
}

/// Notify the runtime of a drawable resize / orientation change in physical pixels.
#[no_mangle]
pub extern "C" fn port_ios_resize_surface(width_px: u32, height_px: u32, scale: f64) {
    win32_rt::platform::resize_host_surface(width_px, height_px, scale);
}

/// Detach the host `CAMetalLayer*` (app backgrounding / view teardown).
#[no_mangle]
pub extern "C" fn port_ios_detach_surface() {
    win32_rt::platform::detach_host_surface();
}

/// Forward a touch event (`phase`: 0=began, 1=moved, 2=ended, 3=cancelled; `x_px`/`y_px`
/// in physical surface pixels) into Win32 mouse input.
#[no_mangle]
pub extern "C" fn port_ios_touch_event(phase: u32, x_px: f32, y_px: f32) {
    win32_rt::platform::touch_event(phase, x_px, y_px);
}

/// Forward a hardware/software keyboard event (`mac_keycode`, key down, repeat, UTF-16 char).
#[no_mangle]
pub extern "C" fn port_ios_key_event(mac_keycode: u16, down: bool, repeat: bool, utf16_char: u16) {
    win32_rt::platform::key_event(mac_keycode, down, repeat, utf16_char);
}

/// Forward UTF-8 text entered via the native iOS software keyboard (`UIKeyInput`) into Win32.
#[no_mangle]
pub unsafe extern "C" fn port_ios_text_input(utf8_text: *const c_char) {
    if utf8_text.is_null() {
        return;
    }
    if let Ok(s) = CStr::from_ptr(utf8_text).to_str() {
        win32_rt::platform::text_input(s);
    }
}

/// Forward an explicit mouse button press/release (`button`: 0=left, 1=right, 2=middle)
/// at physical surface coordinates.
#[no_mangle]
pub extern "C" fn port_ios_mouse_button(button: u32, down: bool, x_px: f32, y_px: f32) {
    win32_rt::platform::mouse_button_event(button, down, x_px, y_px);
}

/// Forward a mouse-wheel scroll delta (Win32 wheel units: +/-120 per notch)
/// at physical surface coordinates into `WM_MOUSEWHEEL` / `WM_MOUSEHWHEEL`.
#[no_mangle]
pub extern "C" fn port_ios_scroll_wheel(x_px: f32, y_px: f32, delta_y: i32, delta_x: i32) {
    win32_rt::platform::scroll_wheel_event(x_px, y_px, delta_y, delta_x);
}

/// Report native iOS hardware & software keyboard presence and frame height (in physical pixels).
#[no_mangle]
pub extern "C" fn port_ios_keyboard_state(
    hardware_connected: bool,
    software_visible: bool,
    software_height_px: u32,
) {
    win32_rt::platform::set_keyboard_state(hardware_connected, software_visible, software_height_px);
}

/// Push the iOS virtual touch-overlay gamepad state directly into `xinput1_4` (slot 0).
#[no_mangle]
pub extern "C" fn port_ios_gamepad_state(
    connected: bool,
    buttons: u16,
    left_trigger: u8,
    right_trigger: u8,
    thumb_lx: i16,
    thumb_ly: i16,
    thumb_rx: i16,
    thumb_ry: i16,
) {
    win32_rt::platform::virtual_gamepad_state(
        connected,
        win32_rt::misc::xinput1_4::Gamepad {
            buttons,
            left_trigger,
            right_trigger,
            thumb_lx,
            thumb_ly,
            thumb_rx,
            thumb_ry,
        },
    );
}

/// Register (or clear with `NULL`) the iOS host phone-haptics callback invoked when
/// a Windows game calls `XInputSetState(0, ...)` and no rumble-capable physical
/// controller owns slot 0 (routes to the iPhone Taptic Engine via `CoreHaptics`).
#[no_mangle]
pub extern "C" fn port_ios_set_rumble_callback(cb: Option<extern "C" fn(u16, u16)>) {
    win32_rt::platform::set_rumble_callback(cb);
}

/// Periodic main-thread tick (call from `CADisplayLink` or a 20 Hz timer) to poll
/// GameController connects/disconnects.
#[no_mangle]
pub extern "C" fn port_ios_tick() {
    win32_rt::platform::gamepad::tick();
}

/// Notify the runtime of an iOS low-memory warning (`didReceiveMemoryWarning`):
/// logs footprint/headroom, trims shader/library caches, flushes pending PSO archives,
/// and signals any registered DXGI video-memory budget change notification events.
#[no_mangle]
pub extern "C" fn port_ios_memory_warning() {
    win32_rt::d3d12::mtl::handle_memory_warning();
}

/// Register a statically linked, signed ARM64 AOT module emitted by the offline
/// Mach-O `.o` compiler (Milestone #2). Safe to call before [`port_ios_start_background`].
#[no_mangle]
pub unsafe extern "C" fn port_ios_register_static_aot(
    module_name: *const c_char,
    code_base: usize,
    entries: *const x86_runtime::emu::exec_mem::StaticAotEntry,
    entry_count: usize,
) {
    if module_name.is_null() || entries.is_null() || entry_count == 0 || code_base == 0 {
        return;
    }
    let Some(name) = opt_cstr(module_name) else {
        return;
    };
    let slice = std::slice::from_raw_parts(entries, entry_count);
    x86_runtime::emu::exec_mem::register_static_aot_module(&name, code_base, slice);
}

/// Copy any declarative `[ios] seed_file` defaults declared in `game.toml` into `drive_c`
/// on first launch (never overwriting files that already exist on disk).
fn seed_ios_default_files(manifest_override: Option<&std::path::Path>, drive_c_override: Option<&std::path::Path>) {
    let manifest_file = manifest_override.map(PathBuf::from).or_else(|| {
        std::env::current_exe()
            .ok()
            .and_then(|exe| exe.parent().map(|d| d.join("game.toml")))
            .filter(|p| p.is_file())
    });
    let Some(mf_path) = manifest_file else {
        return;
    };
    let Ok(manifest) = crate::target::TargetManifest::load_file(&mf_path) else {
        return;
    };
    if manifest.ios_seed_files.is_empty() {
        return;
    }
    let Some(bundle_dir) = mf_path.parent() else {
        return;
    };
    let drive_c = drive_c_override
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("TRANSLATOR_HOME").filter(|v| !v.is_empty()).map(PathBuf::from))
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join("Documents").join("Translator")));
    let Some(drive_c) = drive_c else {
        return;
    };
    for (src_rel, dst_rel) in &manifest.ios_seed_files {
        let src = bundle_dir.join(src_rel);
        let dst = drive_c.join(dst_rel);
        if dst.exists() || !src.is_file() {
            continue;
        }
        if let Some(parent) = dst.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        match std::fs::copy(&src, &dst) {
            Ok(_) => log::info!("port-ios: seeded default {} from {}", dst.display(), src.display()),
            Err(e) => log::warn!("port-ios: failed to seed {}: {e}", dst.display()),
        }
    }
}

/// Start the universal Windows runtime on a dedicated background thread so UIKit's
/// `UIApplicationMain` loop remains responsive.
///
/// Returns `0` if started, `-1` if already started, or `-2` if the boot thread could
/// not be spawned.
#[no_mangle]
pub unsafe extern "C" fn port_ios_start_background(cfg: *const PortIosConfig) -> i32 {
    if STARTED.swap(true, Ordering::AcqRel) {
        return -1;
    }
    let (game_dir, exe_name, manifest_path, drive_c_dir) = if cfg.is_null() {
        (None, None, None, None)
    } else {
        let c = *cfg;
        (
            opt_cstr(c.game_dir).map(PathBuf::from),
            opt_cstr(c.exe_name),
            opt_cstr(c.manifest_path).map(PathBuf::from),
            opt_cstr(c.drive_c_dir).map(PathBuf::from),
        )
    };

    let spawn_res = std::thread::Builder::new()
        .name("port-ios-boot".into())
        .spawn(move || {
            seed_ios_default_files(manifest_path.as_deref(), drive_c_dir.as_deref());
            let mut argv: Vec<String> = Vec::new();
            if let Some(m) = manifest_path {
                argv.push("--manifest".into());
                argv.push(m.to_string_lossy().into_owned());
            }
            if let Some(d) = game_dir {
                argv.push("--game-dir".into());
                argv.push(d.to_string_lossy().into_owned());
            }
            if let Some(e) = exe_name {
                argv.push("--exe".into());
                argv.push(e);
            }
            if let Some(dc) = drive_c_dir {
                argv.push("--drive-c".into());
                argv.push(dc.to_string_lossy().into_owned());
            }
            let args = crate::parse_args_from(argv);
            if let Err(e) = crate::run_embedded(args) {
                log::error!("port-ios fatal startup error: {e:#}");
                STARTED.store(false, Ordering::Release);
            }
        });

    match spawn_res {
        Ok(_) => 0,
        Err(e) => {
            log::error!("port-ios: failed to spawn boot thread: {e}");
            STARTED.store(false, Ordering::Release);
            -2
        }
    }
}

/// Request graceful runtime teardown (flushes D3D12 traces, audio, and exit hooks).
#[no_mangle]
pub extern "C" fn port_ios_request_exit(_code: u32) {
    win32_rt::platform::detach_host_surface();
    win32_rt::process::run_exit_hooks();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_aot_c_abi_registers_cleanly() {
        let name = std::ffi::CString::new("ios_host_test.dll").unwrap();
        let entries = [x86_runtime::emu::exec_mem::StaticAotEntry {
            rva: 0x3000,
            code_offset: 0x80,
            flags: 0,
        }];
        unsafe {
            port_ios_register_static_aot(name.as_ptr(), 0x2_0000_0000, entries.as_ptr(), entries.len());
        }
        let bound = x86_runtime::emu::exec_mem::bind_static_aot_module("ios_host_test.dll", 0x2_4000_0000, 0x10_0000);
        assert_eq!(bound, 1);
        assert_eq!(
            x86_runtime::emu::exec_mem::lookup_static_aot(0x2_4000_3000),
            Some((0x2_0000_0080, false))
        );
    }

    #[test]
    fn gamepad_and_keyboard_c_abi_bridge_works() {
        port_ios_keyboard_state(true, false, 0);
        assert_eq!(
            win32_rt::platform::keyboard_state(),
            win32_rt::platform::KeyboardState {
                hardware_connected: true,
                software_visible: false,
                software_height_px: 0,
            }
        );
        port_ios_keyboard_state(false, true, 840);
        assert_eq!(
            win32_rt::platform::keyboard_state(),
            win32_rt::platform::KeyboardState {
                hardware_connected: false,
                software_visible: true,
                software_height_px: 840,
            }
        );
        port_ios_keyboard_state(false, false, 0);

        port_ios_gamepad_state(true, 0x1001, 64, 255, 10000, -10000, 5000, 20000);
        let (pad, _) = win32_rt::misc::xinput1_4::virtual_gamepad_state().expect("connected");
        assert_eq!(pad.buttons, 0x1001);
        assert_eq!((pad.left_trigger, pad.right_trigger), (64, 255));
        assert_eq!((pad.thumb_lx, pad.thumb_ly, pad.thumb_rx, pad.thumb_ry), (10000, -10000, 5000, 20000));
        port_ios_gamepad_state(false, 0, 0, 0, 0, 0, 0, 0);
        assert!(win32_rt::misc::xinput1_4::virtual_gamepad_state().is_none());

        let txt = std::ffi::CString::new("Hi\n").unwrap();
        unsafe { port_ios_text_input(txt.as_ptr()) };
        port_ios_mouse_button(1, true, 120.0, 240.0);
        port_ios_mouse_button(1, false, 120.0, 240.0);
        port_ios_scroll_wheel(120.0, 240.0, -120, 0);

        use std::sync::atomic::{AtomicU32, Ordering as AtomOrd};
        static LAST_RUMBLE: AtomicU32 = AtomicU32::new(0);
        extern "C" fn on_rumble(l: u16, r: u16) {
            LAST_RUMBLE.store(((l as u32) << 16) | (r as u32), AtomOrd::Release);
        }
        port_ios_set_rumble_callback(Some(on_rumble));
        port_ios_gamepad_state(true, 0, 0, 0, 0, 0, 0, 0);
        let mut vib = win32_rt::misc::xinput1_4::Vibration { left: 0x8000, right: 0x4000 };
        unsafe {
            assert_eq!(win32_rt::misc::xinput1_4::XInputSetState(0, &mut vib), 0);
        }
        assert_eq!(LAST_RUMBLE.load(AtomOrd::Acquire), 0x8000_4000);
        port_ios_gamepad_state(false, 0, 0, 0, 0, 0, 0, 0);
        assert_eq!(LAST_RUMBLE.load(AtomOrd::Acquire), 0);
        port_ios_set_rumble_callback(None);
    }
}
