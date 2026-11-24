//! Alan Wake 2 [`GamePlugin`] implementation, middleware hooks, and opt-in diagnostic tracers.
//!
//! Houses all Alan Wake 2–specific hooks (`cohtml`, `physx3`, `AW2_AUTO_PLAY`,
//! `AW2_CAMERA_TRACE`, replacement registry metadata, and optional `wwise-bridge`
//! post-map hooks) inside `aw2-game` so the universal `x86-runtime` and `port-loader`
//! crates stay 100% game-agnostic.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Mutex;
use win32_rt as w;
use x86_runtime::game_plugin::{
    GamePlugin, GamePluginDefaults, GamePostMapContext, ReplacementSet,
};

/// Alan Wake 2 [`GamePlugin`] implementation.
pub struct Aw2Plugin;

/// Static singleton instance of [`Aw2Plugin`].
pub static AW2_PLUGIN: Aw2Plugin = Aw2Plugin;

/// Register [`AW2_PLUGIN`] with `x86-runtime`'s global plugin registry.
pub fn register_plugin() {
    x86_runtime::game_plugin::register_plugin(&AW2_PLUGIN);
}

/// Default PhysX DLL routes for Alan Wake 2: builtin `physx3_x64.dll` stubs
/// `PxScene::raycast`/`sweep`/`overlap` as `ret0`, so route the game's shipping
/// PhysX 3.4 PEs through guest ARM64 JIT by default.
const AW2_DEFAULT_DLL_ROUTES: &[(&str, &str)] = &[
    ("physx3_x64.dll", "guest"),
    ("physx3common_x64.dll", "guest"),
    ("physx3cooking_x64.dll", "guest"),
    ("pxfoundation_x64.dll", "guest"),
];

/// VAs with non-standard calling conventions that must be excluded from automated hooking.
const AW2_SPECIAL_ABI_VAS: &[u64] = &[
    0x1_4183_53d0,
    0x1_4183_5480,
    0x1_4254_7700,
    0x1_4254_7720,
];

impl GamePlugin for Aw2Plugin {
    fn id(&self) -> &'static str {
        "alan-wake-2"
    }

    fn matches(&self, manifest_id: Option<&str>, exe_norm: &str) -> bool {
        manifest_id.is_some_and(|id| id.eq_ignore_ascii_case("alan-wake-2"))
            || exe_norm.eq_ignore_ascii_case("alanwake2.exe")
    }

    fn defaults(&self) -> GamePluginDefaults {
        GamePluginDefaults {
            app_name: "Alan Wake 2",
            exe: "AlanWake2.exe",
            log_name: "AlanWake2.log",
            log_dir: "AW2Port",
            computer_name: "AW2PORT",
            preferred_base_retry: true,
            legacy_remedy_migration: true,
            v8_jitless: true,
            dll_routes: AW2_DEFAULT_DLL_ROUTES,
        }
    }

    fn on_pre_load(&self) {
        crate::register_aw2_runtime();
        install_middleware_hooks();
    }

    fn replacements(&self) -> Option<ReplacementSet> {
        Some(ReplacementSet {
            base: crate::registry::base().collect(),
            all: crate::registry::all().collect(),
            total_count: crate::registry::count(),
            float_return_vas: crate::registry::FLOAT_RETURN,
            special_abi_vas: AW2_SPECIAL_ABI_VAS,
            preferred_exe_base: 0x1_4000_0000,
        })
    }

    fn on_post_map(&self, ctx: &mut GamePostMapContext<'_>) -> Result<(), String> {
        #[cfg(feature = "wwise-bridge")]
        {
            if ctx.audio_harness {
                // CRT startup runs normally; WinMain (0x141856b20, called from
                // __scrt_common_main_seh) is replaced by the audio test driver.
                const WINMAIN: (u64, u64) = (0x1_4185_6b20, 0x1_4185_84de);
                let at = ctx
                    .exe_base
                    .wrapping_add(WINMAIN.0.wrapping_sub(ctx.preferred_exe_base) as usize);
                let len = (WINMAIN.1 - WINMAIN.0) as usize;
                let target = w::audio::harness::winmain as *const () as usize;
                (ctx.install_hook)(at, Some(at.wrapping_add(len)), target)?;
                log::info!(
                    "--audio-harness: WinMain @0x{at:x} -> win32_rt::audio::harness::winmain"
                );
            }
            w::audio::akrec::install_from_env(ctx.install_hook)?;
            Ok(())
        }
        #[cfg(not(feature = "wwise-bridge"))]
        {
            let _ = ctx;
            Ok(())
        }
    }
}

/// Install all Alan Wake 2–specific middleware hooks (`cohtml`, `physx3`, `AW2_AUTO_PLAY`,
/// and opt-in `AW2_CAMERA_TRACE=1` diagnostics).
fn install_middleware_hooks() {
    w::misc::cohtml_windowsdesktop::set_view_hook(Box::new(Aw2CohtmlHook::default()));
    w::misc::physx3_x64::set_completion_task_hook(aw2_physx_completion_task_hook);
    if std::env::var_os("AW2_AUTO_PLAY").is_some_and(|v| v != "0")
        && std::env::var_os("PORT_INPUT_SCRIPT").is_none()
        && std::env::var_os("AW2_AUTO_PLAY_SCRIPT").is_none()
    {
        w::present::register_present_hook(aw2_auto_play_step);
    }
    if std::env::var_os("AW2_CAMERA_TRACE").is_some_and(|v| v != "0") {
        cam_trace::init();
        w::present::register_present_hook(cam_trace::on_present);
    }
}

// ---------------------------------------------------------------------------
// 1. PhysX 3.4 `PhysXCompletionTask::vfunc_1` (`0x1421437d0`) SRWLOCK bypass
// ---------------------------------------------------------------------------

unsafe fn aw2_physx_completion_task_hook(task: usize, run_fn: usize) -> bool {
    if run_fn != 0x1421437d0 {
        return false;
    }
    // `physics::PhysXCompletionTask::vfunc_1` (`0x1421437d0`): caller already holds `SRWLOCK`
    // at `task + 0x30` while invoking `PxScene::collide`/`PxScene::advance`, and both
    // `fetchCollision` and `fetchResults` on our builtin `PxScene` are immediate no-ops.
    let completed = (task + 0x28) as *const AtomicU8;
    (*completed).store(1, Ordering::Release);
    true
}

// ---------------------------------------------------------------------------
// 2. Coherent Gameface (`cohtml.WindowsDesktop.dll`) AW2 splash/New-Game bridge
// ---------------------------------------------------------------------------

#[derive(Default)]
struct Aw2ViewState {
    setup_fired: bool,
    new_game_fired: bool,
    first_time_setup_completed_handler: usize,
    splash_scroll_bottom_handler: usize,
    splash_scroll_continue_handler: usize,
    on_new_game_selected_handler: usize,
}

#[derive(Default)]
struct Aw2CohtmlHook {
    views: Mutex<HashMap<u32, Aw2ViewState>>,
}

static EMPTY_CSTR: &[u8] = b"\0";

unsafe fn read_inplace_string(addr: usize) -> (*const u8, usize) {
    if addr == 0 {
        return (EMPTY_CSTR.as_ptr(), 0);
    }
    let header = std::ptr::read_unaligned(addr as *const u32);
    let len = std::ptr::read_unaligned((addr + 4) as *const u32) as usize;
    let b3 = (header >> 24) as u32;
    let inline_bits = if b3 < 0x80 { b3 * 8 } else { b3 * 0x100 - 0x7c00 };
    let cap_bits = (header & 0x00ff_ffff) << 3;
    let ptr = if cap_bits > inline_bits {
        std::ptr::read_unaligned((addr + 8) as *const *const u8)
    } else {
        (addr + 8) as *const u8
    };
    if ptr.is_null() {
        (EMPTY_CSTR.as_ptr(), 0)
    } else {
        (ptr, len)
    }
}

fn in_pe_image(addr: usize) -> bool {
    w::try_loader()
        .and_then(|l| l.module_from_address(addr))
        .is_some()
}

unsafe fn fire_new_game_event(handler: usize) -> bool {
    if handler == 0 || !in_pe_image(0x14397da68) {
        return false;
    }
    let root_ptr = std::ptr::read_unaligned(0x14397da68usize as *const usize);
    if root_ptr == 0 {
        return false;
    }
    let ecs_world = std::ptr::read_unaligned((root_ptr + 8) as *const usize);
    if ecs_world == 0 {
        return false;
    }
    let env_table = std::ptr::read_unaligned((ecs_world + 0x580e0) as *const usize);
    if env_table == 0 {
        return false;
    }
    let splash_env_id = std::ptr::read_unaligned(0x143c8cf64usize as *const u16) as usize;
    if splash_env_id == 0xffff {
        return false;
    }
    let splash_env = std::ptr::read_unaligned((env_table + splash_env_id * 8) as *const usize);
    if splash_env == 0 || std::ptr::read_unaligned(splash_env as *const u8) == 0 {
        return false;
    }
    let levels_env_id = std::ptr::read_unaligned(0x143c9033cusize as *const u16) as usize;
    let menu_state_env_id = std::ptr::read_unaligned(0x143c8ced0usize as *const u16) as usize;
    if levels_env_id == 0xffff || menu_state_env_id == 0xffff {
        return false;
    }
    let levels_data = std::ptr::read_unaligned((env_table + levels_env_id * 8) as *const usize);
    let menu_state = std::ptr::read_unaligned((env_table + menu_state_env_id * 8) as *const usize);
    if levels_data == 0 || menu_state == 0 {
        return false;
    }
    let start_pt_count = std::ptr::read_unaligned((levels_data + 0x50) as *const u32);
    let start_pts = std::ptr::read_unaligned((levels_data + 0x48) as *const usize);
    if start_pt_count == 0 || start_pts == 0 {
        return false;
    }
    let (_world_ptr, world_len) = read_inplace_string(levels_data);
    if world_len == 0 {
        return false;
    }
    // Copy (world, session, jump_point) InplaceString<64> fields from UIMainMenuLevelsData
    // into UIMainMenuState (+0x08, +0x90, +0x118) via 0x14240aa00.
    let _ = w::call_guest8(
        0x14240aa00,
        (menu_state + 0x08) as u64,
        levels_data as u64,
        0,
        0,
        0,
        0,
        0,
        0,
    );
    let _ = w::call_guest8(
        0x14240aa00,
        (menu_state + 0x90) as u64,
        start_pts as u64,
        0,
        0,
        0,
        0,
        0,
        0,
    );
    let _ = w::call_guest8(
        0x14240aa00,
        (menu_state + 0x118) as u64,
        (start_pts + 0x48) as u64,
        0,
        0,
        0,
        0,
        0,
        0,
    );
    std::ptr::write_unaligned((menu_state + 0x1a0) as *mut u8, 0);
    std::ptr::write_unaligned((menu_state + 0x1a1) as *mut u8, 0);
    std::ptr::write_unaligned((menu_state + 0x1a2) as *mut u16, 1);
    std::ptr::write_unaligned(menu_state as *mut i32, 1);
    let loading_env_id = std::ptr::read_unaligned(0x143c8cee0usize as *const u16) as usize;
    if loading_env_id != 0xffff {
        let loading_state =
            std::ptr::read_unaligned((env_table + loading_env_id * 8) as *const usize);
        if loading_state != 0 {
            std::ptr::write_unaligned((loading_state + 0x158) as *mut u8, 1);
        }
    }
    log::info!(target: "d3d", "cohtml (aw2 hook): triggered Main Menu -> New Game (Normal) transition");
    true
}

impl w::misc::cohtml_windowsdesktop::CohtmlViewHook for Aw2CohtmlHook {
    unsafe fn on_first_advance(&self, _view_id: u32) {
        if in_pe_image(0x143a344e8) {
            let cmd_opts = std::ptr::read_unaligned(0x143a344e8usize as *const usize);
            if cmd_opts != 0 {
                std::ptr::write_unaligned((cmd_opts + 0x5730) as *mut u8, 1);
            }
        }
    }

    unsafe fn on_advance(&self, view_id: u32, advance_count: u32) {
        if (2..=30).contains(&advance_count) && in_pe_image(0x14397da68) {
            let root_ptr = std::ptr::read_unaligned(0x14397da68usize as *const usize);
            if root_ptr != 0 {
                let ecs_world = std::ptr::read_unaligned((root_ptr + 8) as *const usize);
                if ecs_world != 0 {
                    let env_table = std::ptr::read_unaligned((ecs_world + 0x580e0) as *const usize);
                    let splash_env_id =
                        std::ptr::read_unaligned(0x143c8cf64usize as *const u16) as usize;
                    if env_table != 0 && splash_env_id != 0xffff {
                        let splash_env = std::ptr::read_unaligned(
                            (env_table + splash_env_id * 8) as *const usize,
                        );
                        if splash_env != 0 {
                            std::ptr::write_unaligned((splash_env + 3) as *mut u8, 1);
                        }
                    }
                }
            }
        }
        let (fire_setup, h_bot, h_cont, h_setup, try_new_game) = {
            let map = self.views.lock().unwrap();
            let Some(st) = map.get(&view_id) else {
                return;
            };
            let fs = st.first_time_setup_completed_handler != 0
                && advance_count >= 10
                && advance_count % 30 == 10;
            let ng = if st.on_new_game_selected_handler != 0
                && !st.new_game_fired
                && advance_count >= 90
            {
                st.on_new_game_selected_handler
            } else {
                0
            };
            (
                fs,
                st.splash_scroll_bottom_handler,
                st.splash_scroll_continue_handler,
                st.first_time_setup_completed_handler,
                ng,
            )
        };
        if fire_setup {
            w::misc::cohtml_windowsdesktop::fire_event_0(h_bot);
            w::misc::cohtml_windowsdesktop::fire_event_0(h_cont);
            w::misc::cohtml_windowsdesktop::fire_event_0(h_setup);
        }
        if try_new_game != 0 && fire_new_game_event(try_new_game) {
            if let Some(st) = self.views.lock().unwrap().get_mut(&view_id) {
                st.new_game_fired = true;
            }
        }
    }

    unsafe fn on_register_event(&self, view_id: u32, event_name: &str, handler: usize) {
        let mut map = self.views.lock().unwrap();
        let st = map.entry(view_id).or_default();
        match event_name {
            "first_time_setup_completed" => {
                log::info!(target: "d3d", "cohtml (aw2 hook): registered first_time_setup_completed ({handler:#x})");
                st.first_time_setup_completed_handler = handler;
                st.setup_fired = false;
            }
            "splash_scroll_bottom_reached" => st.splash_scroll_bottom_handler = handler,
            "splash_scroll_continue" => st.splash_scroll_continue_handler = handler,
            "OnNewGameSelected" => {
                log::info!(target: "d3d", "cohtml (aw2 hook): registered OnNewGameSelected ({handler:#x})");
                st.on_new_game_selected_handler = handler;
                st.new_game_fired = false;
            }
            _ => {}
        }
    }

    unsafe fn on_trigger_event(&self, view_id: u32, advance_count: u32, _event_name: Option<&str>) {
        let (fire_setup, h_bot, h_cont, h_setup, try_new_game) = {
            let mut map = self.views.lock().unwrap();
            let Some(st) = map.get_mut(&view_id) else {
                return;
            };
            let fs = if st.first_time_setup_completed_handler != 0 && !st.setup_fired {
                st.setup_fired = true;
                true
            } else {
                false
            };
            let ng = if st.on_new_game_selected_handler != 0
                && !st.new_game_fired
                && advance_count >= 20
            {
                st.on_new_game_selected_handler
            } else {
                0
            };
            (
                fs,
                st.splash_scroll_bottom_handler,
                st.splash_scroll_continue_handler,
                st.first_time_setup_completed_handler,
                ng,
            )
        };
        if fire_setup {
            w::misc::cohtml_windowsdesktop::fire_event_0(h_bot);
            w::misc::cohtml_windowsdesktop::fire_event_0(h_cont);
            w::misc::cohtml_windowsdesktop::fire_event_0(h_setup);
        }
        if try_new_game != 0 && fire_new_game_event(try_new_game) {
            if let Some(st) = self.views.lock().unwrap().get_mut(&view_id) {
                st.new_game_fired = true;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 3. `AW2_AUTO_PLAY` menu & cutscene skip driver (registered via PresentHook)
// ---------------------------------------------------------------------------

fn aw2_auto_play_step(hwnd: usize, frame: u32) {
    if cam_trace::walk_overrides_autoplay() {
        return;
    }
    use w::misc::xinput1_4::{self, btn, DeviceInfo, Gamepad};
    use w::user32::input::{self, MouseButton, NativeKey, NativeMouse};
    use w::user32::POINT;
    input::force_focus_for_autoplay(hwnd);
    if frame == 30 {
        xinput1_4::backend_connect(
            0,
            DeviceInfo {
                name: "AW2 Virtual Controller".into(),
                wireless: false,
                rumble: false,
                battery: None,
            },
        );
    }
    if frame >= 60 {
        let phase = (frame - 60) % 45;
        if frame < 560 {
            let pt = POINT { x: 640, y: 360 };
            if phase == 0 {
                input::native_key(NativeKey {
                    mac: 0x24,
                    down: true,
                    repeat: false,
                    chars: vec![0x0D],
                });
                input::native_key(NativeKey {
                    mac: 0x31,
                    down: true,
                    repeat: false,
                    chars: vec![0x20],
                });
                input::native_key(NativeKey {
                    mac: 0x0E,
                    down: true,
                    repeat: false,
                    chars: vec![b'e' as u16],
                });
                if hwnd != 0 {
                    input::native_mouse(NativeMouse::Move {
                        hwnd,
                        client: Some(pt),
                        screen: pt,
                        dx: 1.0,
                        dy: 0.0,
                    });
                    input::native_mouse(NativeMouse::Button {
                        hwnd,
                        client: Some(pt),
                        screen: pt,
                        button: MouseButton::Left,
                        down: true,
                        clicks: 1,
                    });
                }
                xinput1_4::backend_update(
                    0,
                    Gamepad {
                        buttons: btn::A,
                        ..Default::default()
                    },
                );
            } else if phase == 6 {
                input::native_key(NativeKey {
                    mac: 0x24,
                    down: false,
                    repeat: false,
                    chars: Vec::new(),
                });
                input::native_key(NativeKey {
                    mac: 0x31,
                    down: false,
                    repeat: false,
                    chars: vec![],
                });
                input::native_key(NativeKey {
                    mac: 0x0E,
                    down: false,
                    repeat: false,
                    chars: vec![],
                });
                if hwnd != 0 {
                    input::native_mouse(NativeMouse::Button {
                        hwnd,
                        client: Some(pt),
                        screen: pt,
                        button: MouseButton::Left,
                        down: false,
                        clicks: 1,
                    });
                }
                xinput1_4::backend_update(0, Gamepad::default());
            }
        } else if frame < 760 {
            if phase == 0 {
                input::native_key(NativeKey {
                    mac: 0x7D,
                    down: true,
                    repeat: false,
                    chars: Vec::new(),
                });
                xinput1_4::backend_update(
                    0,
                    Gamepad {
                        buttons: btn::DPAD_DOWN,
                        ..Default::default()
                    },
                );
            } else if phase == 5 {
                input::native_key(NativeKey {
                    mac: 0x7D,
                    down: false,
                    repeat: false,
                    chars: Vec::new(),
                });
                xinput1_4::backend_update(0, Gamepad::default());
            } else if phase == 15 {
                input::native_key(NativeKey {
                    mac: 0x24,
                    down: true,
                    repeat: false,
                    chars: vec![0x0D],
                });
                xinput1_4::backend_update(
                    0,
                    Gamepad {
                        buttons: btn::A,
                        ..Default::default()
                    },
                );
            } else if phase == 22 {
                input::native_key(NativeKey {
                    mac: 0x24,
                    down: false,
                    repeat: false,
                    chars: Vec::new(),
                });
                xinput1_4::backend_update(0, Gamepad::default());
            }
        } else {
            let skip_phase = (frame - 760) % 140;
            if skip_phase == 0 {
                input::native_key(NativeKey {
                    mac: 0x24,
                    down: true,
                    repeat: false,
                    chars: vec![0x0D],
                });
                input::native_key(NativeKey {
                    mac: 0x31,
                    down: true,
                    repeat: false,
                    chars: vec![0x20],
                });
                xinput1_4::backend_update(
                    0,
                    Gamepad {
                        buttons: btn::A,
                        ..Default::default()
                    },
                );
            } else if skip_phase == 120 {
                input::native_key(NativeKey {
                    mac: 0x24,
                    down: false,
                    repeat: false,
                    chars: Vec::new(),
                });
                input::native_key(NativeKey {
                    mac: 0x31,
                    down: false,
                    repeat: false,
                    chars: Vec::new(),
                });
                xinput1_4::backend_update(0, Gamepad::default());
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 4. Opt-in `AW2_CAMERA_TRACE=1` movement / camera / PhysX diagnostic tracer
// ---------------------------------------------------------------------------

mod cam_trace {
    use super::in_pe_image;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};
    use std::sync::{Mutex, OnceLock};
    use win32_rt as w;
    use x86_runtime::exe;

    const CAM_STATE_PTR_VA: u64 = 0x1_43a0_01d0;
    const ORIG_TAG: u64 = 1u64 << 46;

    const VA_CC_STEP: u64 = 0x1_4217_ab60;
    const VA_FILL_CACHE: u64 = 0x1_4216_f470;
    const VA_CACHE_CB: u64 = 0x1_4216_c430;
    const VA_RAYCAST: u64 = 0x1_4214_f5d0;
    const VA_SWEEP: u64 = 0x1_4214_ed80;
    const VA_OVERLAP: u64 = 0x1_4214_e330;
    const VA_FILTER_PRE: u64 = 0x1_4124_8880;
    const VA_FILTER_POST: u64 = 0x1_4124_8960;
    const VA_CC_FILTER_PRE: u64 = 0x1_4128_c3a0;
    const VA_SDF_SOLVE: u64 = 0x1_4212_c3e0;
    const VA_CAPS_SWEEP: u64 = 0x1_4217_33f0;
    const VA_CULL_LOD: u64 = 0x1_4225_9370;
    const VA_CHROMA_INIT: u64 = 0x1_4250_7820;

    static HOOKS_INSTALLED: AtomicBool = AtomicBool::new(false);
    static WALK_OVERRIDES_AUTOPLAY: AtomicBool = AtomicBool::new(false);

    static ORIG_CC_STEP: AtomicUsize = AtomicUsize::new(0);
    static ORIG_FILL_CACHE: AtomicUsize = AtomicUsize::new(0);
    static ORIG_CACHE_CB: AtomicUsize = AtomicUsize::new(0);
    static ORIG_RAYCAST: AtomicUsize = AtomicUsize::new(0);
    static ORIG_SWEEP: AtomicUsize = AtomicUsize::new(0);
    static ORIG_OVERLAP: AtomicUsize = AtomicUsize::new(0);
    static ORIG_FILTER_PRE: AtomicUsize = AtomicUsize::new(0);
    static ORIG_FILTER_POST: AtomicUsize = AtomicUsize::new(0);
    static ORIG_CC_FILTER_PRE: AtomicUsize = AtomicUsize::new(0);
    static ORIG_SDF_SOLVE: AtomicUsize = AtomicUsize::new(0);
    static ORIG_CAPS_SWEEP: AtomicUsize = AtomicUsize::new(0);
    static ORIG_CULL_LOD: AtomicUsize = AtomicUsize::new(0);

    static LAST_CAM_PTR: AtomicUsize = AtomicUsize::new(0);
    static LAST_CAM_PX: AtomicU64 = AtomicU64::new(0);
    static LAST_CAM_PY: AtomicU64 = AtomicU64::new(0);
    static LAST_CAM_PZ: AtomicU64 = AtomicU64::new(0);
    static LAST_CAM_DX: AtomicU64 = AtomicU64::new(0);
    static LAST_CAM_DY: AtomicU64 = AtomicU64::new(0);
    static LAST_CAM_DZ: AtomicU64 = AtomicU64::new(0);

    // Per-frame atomic counters (swapped to 0 on each Present).
    static CC_CALLS: AtomicU32 = AtomicU32::new(0);
    static CC_P9_CALLS: AtomicU32 = AtomicU32::new(0);
    static CC_P9_ZERO_CALLS: AtomicU32 = AtomicU32::new(0);
    static CC_TOTAL_CALLS: AtomicU64 = AtomicU64::new(0);

    static CCACHE_CALLS: AtomicU32 = AtomicU32::new(0);
    static CCACHE_NONEMPTY: AtomicU32 = AtomicU32::new(0);
    static CCACHE_PRIMS: AtomicU32 = AtomicU32::new(0);
    static CCACHE_MESHES: AtomicU32 = AtomicU32::new(0);
    static CCACHE_TRIS: AtomicU32 = AtomicU32::new(0);
    static CCACHE_TRIS_MAX: AtomicU32 = AtomicU32::new(0);
    static CCACHE_CB_CALLS: AtomicU32 = AtomicU32::new(0);
    static CCACHE_CB_TOUCHES: AtomicU32 = AtomicU32::new(0);

    static RAY_CALLS: AtomicU32 = AtomicU32::new(0);
    static RAY_HITS: AtomicU32 = AtomicU32::new(0);
    static DOWN_RAY_CALLS: AtomicU32 = AtomicU32::new(0);
    static DOWN_RAY_HITS: AtomicU32 = AtomicU32::new(0);
    static SWEEP_CALLS: AtomicU32 = AtomicU32::new(0);
    static SWEEP_HITS: AtomicU32 = AtomicU32::new(0);
    static OVERLAP_CALLS: AtomicU32 = AtomicU32::new(0);
    static OVERLAP_HITS: AtomicU32 = AtomicU32::new(0);

    static FILTER_PRE_CALLS: AtomicU32 = AtomicU32::new(0);
    static FILTER_PRE_NZ: AtomicU32 = AtomicU32::new(0);
    static FILTER_POST_CALLS: AtomicU32 = AtomicU32::new(0);
    static CC_FILTER_PRE_CALLS: AtomicU32 = AtomicU32::new(0);
    static CC_FILTER_PRE_NZ: AtomicU32 = AtomicU32::new(0);

    static SDF_CALLS: AtomicU32 = AtomicU32::new(0);
    static CAPS_CALLS: AtomicU32 = AtomicU32::new(0);

    // Latest CharacterController snapshot (lock-free atomics).
    static LAST_CC_PTR: AtomicUsize = AtomicUsize::new(0);
    static LAST_PX_SCENE: AtomicUsize = AtomicUsize::new(0);
    static LAST_POS_IN_X: AtomicU32 = AtomicU32::new(0);
    static LAST_POS_IN_Y: AtomicU32 = AtomicU32::new(0);
    static LAST_POS_IN_Z: AtomicU32 = AtomicU32::new(0);
    static LAST_POS_OUT_X: AtomicU32 = AtomicU32::new(0);
    static LAST_POS_OUT_Y: AtomicU32 = AtomicU32::new(0);
    static LAST_POS_OUT_Z: AtomicU32 = AtomicU32::new(0);
    static LAST_VEL_IN_X: AtomicU32 = AtomicU32::new(0);
    static LAST_VEL_IN_Y: AtomicU32 = AtomicU32::new(0);
    static LAST_VEL_IN_Z: AtomicU32 = AtomicU32::new(0);
    static LAST_VEL_OUT_X: AtomicU32 = AtomicU32::new(0);
    static LAST_VEL_OUT_Y: AtomicU32 = AtomicU32::new(0);
    static LAST_VEL_OUT_Z: AtomicU32 = AtomicU32::new(0);
    static LAST_CC_BLOCKED: AtomicU32 = AtomicU32::new(0);
    static LAST_CC_ON_GROUND: AtomicU32 = AtomicU32::new(0);
    static LAST_CC_STATUS: AtomicU32 = AtomicU32::new(0);
    static LAST_CC_P9: AtomicU32 = AtomicU32::new(0);
    static LAST_CC_DT: AtomicU32 = AtomicU32::new(0);
    static LAST_SDF_B60: AtomicU32 = AtomicU32::new(0xff);
    static LAST_CAPS_B60: AtomicU32 = AtomicU32::new(0xff);

    // Latest downward raycast snapshot.
    static LAST_DRAY_SY: AtomicU32 = AtomicU32::new(0);
    static LAST_DRAY_EY: AtomicU32 = AtomicU32::new(0);
    static LAST_DRAY_HY: AtomicU32 = AtomicU32::new(0);
    static LAST_DRAY_HIT: AtomicU32 = AtomicU32::new(0);

    #[derive(Clone, Copy)]
    enum WalkMode {
        None,
        Frames { start: u32, end: u32 },
        Auto { delay: u32, dur: u32 },
    }

    struct Config {
        verbose: bool,
        walk: WalkMode,
        skip: Option<(u32, u32)>,
        exit_at: Option<u32>,
        unhook: Vec<u64>,
        log_file: Option<Mutex<std::fs::File>>,
    }

    struct WalkState {
        active_cc_frames: u32,
        started_frame: Option<u32>,
        w_down: bool,
        skip_down: bool,
        gameplay_detected: bool,
        last_cam_y: f64,
    }

    static CFG: OnceLock<Config> = OnceLock::new();
    static WALK_ST: OnceLock<Mutex<WalkState>> = OnceLock::new();

    pub fn walk_overrides_autoplay() -> bool {
        WALK_OVERRIDES_AUTOPLAY.load(Ordering::Relaxed)
    }

    fn parse_walk(raw: &str) -> WalkMode {
        let s = raw.trim();
        if let Some(rest) = s.strip_prefix("auto:") {
            let mut it = rest.split(':');
            if let (Some(d), Some(dur)) = (it.next(), it.next()) {
                if let (Ok(delay), Ok(dur)) = (d.parse::<u32>(), dur.parse::<u32>()) {
                    return WalkMode::Auto { delay, dur };
                }
            }
        } else if let Some((a, b)) = s.split_once(':') {
            if let (Ok(start), Ok(end)) = (a.parse::<u32>(), b.parse::<u32>()) {
                return WalkMode::Frames { start, end };
            }
        }
        WalkMode::None
    }

    pub fn init() {
        let verbose = std::env::var("AW2_CAMERA_TRACE").is_ok_and(|v| v.trim() == "2");
        let walk = std::env::var("AW2_CAMERA_TRACE_WALK")
            .map(|v| parse_walk(&v))
            .unwrap_or(WalkMode::None);
        let skip = std::env::var("AW2_CAMERA_TRACE_SKIP").ok().and_then(|v| {
            let (a, b) = v.trim().split_once(':')?;
            Some((a.parse::<u32>().ok()?, b.parse::<u32>().ok()?))
        });
        let exit_at = std::env::var("AW2_CAMERA_TRACE_EXIT_AT")
            .ok()
            .and_then(|v| v.trim().parse::<u32>().ok());
        let unhook: Vec<u64> = std::env::var("AW2_CAMERA_TRACE_UNHOOK")
            .unwrap_or_default()
            .split(',')
            .filter_map(|tok| {
                let t = tok.trim().trim_start_matches("0x").trim_start_matches("0X");
                if t.is_empty() {
                    None
                } else {
                    u64::from_str_radix(t, 16).ok()
                }
            })
            .collect();
        let log_file = std::env::var("AW2_CAMERA_TRACE_FILE")
            .ok()
            .filter(|p| !p.trim().is_empty())
            .and_then(|p| {
                OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(&p)
                    .map(Mutex::new)
                    .ok()
            });
        let _ = CFG.set(Config {
            verbose,
            walk,
            skip,
            exit_at,
            unhook,
            log_file,
        });
        let _ = WALK_ST.set(Mutex::new(WalkState {
            active_cc_frames: 0,
            started_frame: None,
            w_down: false,
            skip_down: false,
            gameplay_detected: false,
            last_cam_y: f64::NAN,
        }));
    }

    fn emit_line(line: &str) {
        log::info!(target: "cam_trace", "{line}");
        if let Some(cfg) = CFG.get() {
            if let Some(ref fm) = cfg.log_file {
                if let Ok(mut f) = fm.lock() {
                    let _ = writeln!(f, "{line}");
                    let _ = f.flush();
                }
            }
        }
    }

    #[inline(always)]
    unsafe fn call_orig8(
        orig_slot: &AtomicUsize,
        va: u64,
        a1: u64,
        a2: u64,
        a3: u64,
        a4: u64,
        a5: u64,
        a6: u64,
        a7: u64,
        a8: u64,
    ) -> u64 {
        let orig = orig_slot.load(Ordering::Acquire);
        if orig != 0 {
            let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =
                core::mem::transmute(orig);
            f(a1, a2, a3, a4, a5, a6, a7, a8)
        } else {
            exe::call_va8(va | ORIG_TAG, a1, a2, a3, a4, a5, a6, a7, a8)
        }
    }

    unsafe extern "win64" fn wrap_cc_step(
        a1: u64,
        a2: u64,
        a3: u64,
        a4: u64,
        a5: u64,
        a6: u64,
        a7: u64,
        a8: u64,
        a9: u64,
        a10: u64,
        a11: u64,
    ) -> u64 {
        CC_CALLS.fetch_add(1, Ordering::Relaxed);
        CC_TOTAL_CALLS.fetch_add(1, Ordering::Relaxed);
        let p9 = (a9 as u8) as u32;
        if p9 != 0 {
            CC_P9_CALLS.fetch_add(1, Ordering::Relaxed);
        } else {
            CC_P9_ZERO_CALLS.fetch_add(1, Ordering::Relaxed);
        }

        let p1 = a1 as usize;
        let p4 = a4 as usize;
        let pos_in = if p1 != 0 {
            exe::rd::<[u32; 3]>(p1 + 0xa0)
        } else {
            [0; 3]
        };
        let vel_in = if p4 != 0 {
            exe::rd::<[u32; 3]>(p4)
        } else {
            [0; 3]
        };

        let orig = ORIG_CC_STEP.load(Ordering::Acquire);
        let ret = if orig != 0 {
            let f: unsafe extern "win64" fn(
                u64,
                u64,
                u64,
                u64,
                u64,
                u64,
                u64,
                u64,
                u64,
                u64,
                u64,
            ) -> u64 = core::mem::transmute(orig);
            f(a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11)
        } else {
            exe::call_va12(
                VA_CC_STEP | ORIG_TAG,
                a1,
                a2,
                a3,
                a4,
                a5,
                a6,
                a7,
                a8,
                a9,
                a10,
                a11,
                0,
            )
        };

        // Prefer recording the player (`p9 != 0`), or any controller if no `p9 != 0` seen yet.
        let prev_p9 = LAST_CC_P9.load(Ordering::Relaxed);
        if p1 != 0 && (p9 != 0 || prev_p9 == 0) {
            let pos_out = exe::rd::<[u32; 3]>(p1 + 0xa0);
            let vel_out = exe::rd::<[u32; 3]>(p1 + 0x50);
            let blocked = exe::rd::<u8>(p1 + 0xb0) as u32;
            let on_ground = exe::rd::<u8>(p1 + 0x19) as u32;
            let status = exe::rd::<u32>(p1 + 0xb4);
            let p78 = exe::rd::<usize>(p1 + 0x78);
            let px_scene = if p78 != 0 {
                exe::rd::<usize>(p78)
            } else {
                0
            };
            LAST_CC_PTR.store(p1, Ordering::Relaxed);
            LAST_PX_SCENE.store(px_scene, Ordering::Relaxed);
            LAST_POS_IN_X.store(pos_in[0], Ordering::Relaxed);
            LAST_POS_IN_Y.store(pos_in[1], Ordering::Relaxed);
            LAST_POS_IN_Z.store(pos_in[2], Ordering::Relaxed);
            LAST_POS_OUT_X.store(pos_out[0], Ordering::Relaxed);
            LAST_POS_OUT_Y.store(pos_out[1], Ordering::Relaxed);
            LAST_POS_OUT_Z.store(pos_out[2], Ordering::Relaxed);
            LAST_VEL_IN_X.store(vel_in[0], Ordering::Relaxed);
            LAST_VEL_IN_Y.store(vel_in[1], Ordering::Relaxed);
            LAST_VEL_IN_Z.store(vel_in[2], Ordering::Relaxed);
            LAST_VEL_OUT_X.store(vel_out[0], Ordering::Relaxed);
            LAST_VEL_OUT_Y.store(vel_out[1], Ordering::Relaxed);
            LAST_VEL_OUT_Z.store(vel_out[2], Ordering::Relaxed);
            LAST_CC_BLOCKED.store(blocked, Ordering::Relaxed);
            LAST_CC_ON_GROUND.store(on_ground, Ordering::Relaxed);
            LAST_CC_STATUS.store(status, Ordering::Relaxed);
            LAST_CC_P9.store(p9, Ordering::Relaxed);
            LAST_CC_DT.store(a6 as u32, Ordering::Relaxed);
        }
        ret
    }

    unsafe extern "win64" fn wrap_fill_cache(
        a1: u64,
        a2: u64,
        a3: u64,
        a4: u64,
        a5: u64,
        a6: u64,
        a7: u64,
        a8: u64,
    ) -> u64 {
        let ret = call_orig8(&ORIG_FILL_CACHE, VA_FILL_CACHE, a1, a2, a3, a4, a5, a6, a7, a8);
        CCACHE_CALLS.fetch_add(1, Ordering::Relaxed);
        let cache = a1 as usize;
        if cache != 0 {
            let prims = exe::rd::<u32>(cache + 0x08);
            let meshes = exe::rd::<u32>(cache + 0x18);
            let tris = exe::rd::<u32>(cache + 0x28);
            if prims != 0 || meshes != 0 || tris != 0 {
                CCACHE_NONEMPTY.fetch_add(1, Ordering::Relaxed);
            }
            CCACHE_PRIMS.fetch_add(prims, Ordering::Relaxed);
            CCACHE_MESHES.fetch_add(meshes, Ordering::Relaxed);
            CCACHE_TRIS.fetch_add(tris, Ordering::Relaxed);
            CCACHE_TRIS_MAX.fetch_max(tris, Ordering::Relaxed);
        }
        ret
    }

    unsafe extern "win64" fn wrap_cache_cb(
        a1: u64,
        a2: u64,
        a3: u64,
        a4: u64,
        a5: u64,
        a6: u64,
        a7: u64,
        a8: u64,
    ) -> u64 {
        CCACHE_CB_CALLS.fetch_add(1, Ordering::Relaxed);
        CCACHE_CB_TOUCHES.fetch_add(a3 as u32, Ordering::Relaxed);
        call_orig8(&ORIG_CACHE_CB, VA_CACHE_CB, a1, a2, a3, a4, a5, a6, a7, a8)
    }

    unsafe extern "win64" fn wrap_raycast(
        a1: u64,
        a2: u64,
        a3: u64,
        a4: u64,
        a5: u64,
        a6: u64,
        a7: u64,
        a8: u64,
    ) -> u64 {
        let start = if a2 != 0 {
            exe::rd::<[f32; 3]>(a2 as usize)
        } else {
            [0.0; 3]
        };
        let end = if a3 != 0 {
            exe::rd::<[f32; 3]>(a3 as usize)
        } else {
            [0.0; 3]
        };
        let ret = call_orig8(&ORIG_RAYCAST, VA_RAYCAST, a1, a2, a3, a4, a5, a6, a7, a8);
        let hit = (ret & 0xff) != 0;
        RAY_CALLS.fetch_add(1, Ordering::Relaxed);
        if hit {
            RAY_HITS.fetch_add(1, Ordering::Relaxed);
        }
        if a2 != 0 && a3 != 0 && end[1] < start[1] - 0.1 {
            DOWN_RAY_CALLS.fetch_add(1, Ordering::Relaxed);
            if hit {
                DOWN_RAY_HITS.fetch_add(1, Ordering::Relaxed);
                let hy = if a5 != 0 {
                    exe::rd::<f32>((a5 as usize) + 0x30)
                } else {
                    0.0
                };
                LAST_DRAY_HY.store(hy.to_bits(), Ordering::Relaxed);
                LAST_DRAY_HIT.store(1, Ordering::Relaxed);
            } else {
                LAST_DRAY_HIT.store(0, Ordering::Relaxed);
            }
            LAST_DRAY_SY.store(start[1].to_bits(), Ordering::Relaxed);
            LAST_DRAY_EY.store(end[1].to_bits(), Ordering::Relaxed);
        }
        ret
    }

    unsafe extern "win64" fn wrap_sweep(
        a1: u64,
        a2: u64,
        a3: u64,
        a4: u64,
        a5: u64,
        a6: u64,
        a7: u64,
        a8: u64,
    ) -> u64 {
        let ret = call_orig8(&ORIG_SWEEP, VA_SWEEP, a1, a2, a3, a4, a5, a6, a7, a8);
        SWEEP_CALLS.fetch_add(1, Ordering::Relaxed);
        if (ret & 0xff) != 0 {
            SWEEP_HITS.fetch_add(1, Ordering::Relaxed);
        }
        ret
    }

    unsafe extern "win64" fn wrap_overlap(
        a1: u64,
        a2: u64,
        a3: u64,
        a4: u64,
        a5: u64,
        a6: u64,
        a7: u64,
        a8: u64,
    ) -> u64 {
        let ret = call_orig8(&ORIG_OVERLAP, VA_OVERLAP, a1, a2, a3, a4, a5, a6, a7, a8);
        OVERLAP_CALLS.fetch_add(1, Ordering::Relaxed);
        OVERLAP_HITS.fetch_add(ret as u32, Ordering::Relaxed);
        ret
    }

    unsafe extern "win64" fn wrap_filter_pre(
        a1: u64,
        a2: u64,
        a3: u64,
        a4: u64,
        a5: u64,
        a6: u64,
        a7: u64,
        a8: u64,
    ) -> u64 {
        let ret = call_orig8(&ORIG_FILTER_PRE, VA_FILTER_PRE, a1, a2, a3, a4, a5, a6, a7, a8);
        FILTER_PRE_CALLS.fetch_add(1, Ordering::Relaxed);
        if (ret as u32) != 0 {
            FILTER_PRE_NZ.fetch_add(1, Ordering::Relaxed);
        }
        ret
    }

    unsafe extern "win64" fn wrap_filter_post(
        a1: u64,
        a2: u64,
        a3: u64,
        a4: u64,
        a5: u64,
        a6: u64,
        a7: u64,
        a8: u64,
    ) -> u64 {
        let ret = call_orig8(
            &ORIG_FILTER_POST,
            VA_FILTER_POST,
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
            a8,
        );
        FILTER_POST_CALLS.fetch_add(1, Ordering::Relaxed);
        ret
    }

    unsafe extern "win64" fn wrap_cc_filter_pre(
        a1: u64,
        a2: u64,
        a3: u64,
        a4: u64,
        a5: u64,
        a6: u64,
        a7: u64,
        a8: u64,
    ) -> u64 {
        let ret = call_orig8(
            &ORIG_CC_FILTER_PRE,
            VA_CC_FILTER_PRE,
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
            a8,
        );
        CC_FILTER_PRE_CALLS.fetch_add(1, Ordering::Relaxed);
        if (ret as u32) != 0 {
            CC_FILTER_PRE_NZ.fetch_add(1, Ordering::Relaxed);
        }
        ret
    }

    unsafe extern "win64" fn wrap_sdf_solve(
        a1: u64,
        a2: u64,
        a3: u64,
        a4: u64,
        a5: u64,
        a6: u64,
        a7: u64,
        a8: u64,
    ) -> u64 {
        let ret = call_orig8(&ORIG_SDF_SOLVE, VA_SDF_SOLVE, a1, a2, a3, a4, a5, a6, a7, a8);
        SDF_CALLS.fetch_add(1, Ordering::Relaxed);
        if ret != 0 {
            LAST_SDF_B60.store(exe::rd::<u8>((ret as usize) + 0x60) as u32, Ordering::Relaxed);
        }
        ret
    }

    unsafe extern "win64" fn wrap_caps_sweep(
        a1: u64,
        a2: u64,
        a3: u64,
        a4: u64,
        a5: u64,
        a6: u64,
        a7: u64,
        a8: u64,
    ) -> u64 {
        let ret = call_orig8(
            &ORIG_CAPS_SWEEP,
            VA_CAPS_SWEEP,
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
            a8,
        );
        CAPS_CALLS.fetch_add(1, Ordering::Relaxed);
        if ret != 0 {
            LAST_CAPS_B60.store(
                exe::rd::<u8>((ret as usize) + 0x60) as u32,
                Ordering::Relaxed,
            );
        }
        ret
    }

    unsafe extern "win64" fn wrap_cull_lod(
        a1: u64,
        a2: u64,
        a3: u64,
        a4: u64,
        a5: u64,
        a6: u64,
        a7: u64,
        a8: u64,
    ) -> u64 {
        let slot = exe::ptr::<usize>(CAM_STATE_PTR_VA) as usize;
        let cp = exe::rd::<usize>(slot);
        if cp != 0 {
            LAST_CAM_PTR.store(cp, Ordering::Relaxed);
            LAST_CAM_DX.store(exe::rd::<u64>(cp + 0xa0), Ordering::Relaxed);
            LAST_CAM_DY.store(exe::rd::<u64>(cp + 0xa8), Ordering::Relaxed);
            LAST_CAM_DZ.store(exe::rd::<u64>(cp + 0xb0), Ordering::Relaxed);
            LAST_CAM_PX.store(exe::rd::<u64>(cp + 0xb8), Ordering::Relaxed);
            LAST_CAM_PY.store(exe::rd::<u64>(cp + 0xc0), Ordering::Relaxed);
            LAST_CAM_PZ.store(exe::rd::<u64>(cp + 0xc8), Ordering::Relaxed);
        }
        call_orig8(&ORIG_CULL_LOD, VA_CULL_LOD, a1, a2, a3, a4, a5, a6, a7, a8)
    }

    unsafe extern "win64" fn skip_chroma_init() -> u64 {
        0x1787 // RZRESULT_FILE_NOT_FOUND (Chroma Editor Library unavailable without mfc140u.dll)
    }

    fn install_va_wrappers() {
        if HOOKS_INSTALLED.swap(true, Ordering::AcqRel) {
            return;
        }
        if std::env::var_os("AW2_SKIP_CHROMA").is_some_and(|v| v != "0") {
            exe::register_native_va(VA_CHROMA_INIT, skip_chroma_init as *const () as usize);
            emit_line("AW2_SKIP_CHROMA=1: ChromaAnimationAPI::InitSDK -> 0x1787");
        }
        if let Some(cfg) = CFG.get() {
            for &va in &cfg.unhook {
                exe::unregister_native_va(va);
                emit_line(&format!("unhooked VA {va:#x} -> guest JIT"));
            }
        }
        let wrap = |va: u64, slot: &AtomicUsize, wrapper: usize, name: &str| {
            let orig = exe::lookup_va_only(va).unwrap_or(0);
            slot.store(orig, Ordering::Release);
            exe::register_native_va(va, wrapper);
            emit_line(&format!(
                "hooked {name} @ {va:#x} (orig={orig:#x}, mode={})",
                if orig != 0 { "rust" } else { "jit" }
            ));
        };
        wrap(
            VA_CC_STEP,
            &ORIG_CC_STEP,
            wrap_cc_step as *const () as usize,
            "cc_step",
        );
        wrap(
            VA_FILL_CACHE,
            &ORIG_FILL_CACHE,
            wrap_fill_cache as *const () as usize,
            "fill_cache",
        );
        wrap(
            VA_CACHE_CB,
            &ORIG_CACHE_CB,
            wrap_cache_cb as *const () as usize,
            "cache_cb",
        );
        wrap(
            VA_RAYCAST,
            &ORIG_RAYCAST,
            wrap_raycast as *const () as usize,
            "raycast",
        );
        wrap(
            VA_SWEEP,
            &ORIG_SWEEP,
            wrap_sweep as *const () as usize,
            "sweep",
        );
        wrap(
            VA_OVERLAP,
            &ORIG_OVERLAP,
            wrap_overlap as *const () as usize,
            "overlap",
        );
        wrap(
            VA_FILTER_PRE,
            &ORIG_FILTER_PRE,
            wrap_filter_pre as *const () as usize,
            "filter_pre",
        );
        wrap(
            VA_FILTER_POST,
            &ORIG_FILTER_POST,
            wrap_filter_post as *const () as usize,
            "filter_post",
        );
        wrap(
            VA_CC_FILTER_PRE,
            &ORIG_CC_FILTER_PRE,
            wrap_cc_filter_pre as *const () as usize,
            "cc_filter_pre",
        );
        wrap(
            VA_SDF_SOLVE,
            &ORIG_SDF_SOLVE,
            wrap_sdf_solve as *const () as usize,
            "sdf_solve",
        );
        wrap(
            VA_CAPS_SWEEP,
            &ORIG_CAPS_SWEEP,
            wrap_caps_sweep as *const () as usize,
            "caps_sweep",
        );
        wrap(
            VA_CULL_LOD,
            &ORIG_CULL_LOD,
            wrap_cull_lod as *const () as usize,
            "cull_lod",
        );
    }

    pub fn on_present(hwnd: usize, frame: u32) {
        if frame == 1 || !HOOKS_INSTALLED.load(Ordering::Acquire) {
            install_va_wrappers();
        }
        let Some(cfg) = CFG.get() else {
            return;
        };

        let cc_calls = CC_CALLS.swap(0, Ordering::Relaxed);
        let cc_p9 = CC_P9_CALLS.swap(0, Ordering::Relaxed);
        let cc_p9z = CC_P9_ZERO_CALLS.swap(0, Ordering::Relaxed);
        let ccache_calls = CCACHE_CALLS.swap(0, Ordering::Relaxed);
        let ccache_ne = CCACHE_NONEMPTY.swap(0, Ordering::Relaxed);
        let ccache_prims = CCACHE_PRIMS.swap(0, Ordering::Relaxed);
        let ccache_meshes = CCACHE_MESHES.swap(0, Ordering::Relaxed);
        let ccache_tris = CCACHE_TRIS.swap(0, Ordering::Relaxed);
        let ccache_tris_max = CCACHE_TRIS_MAX.swap(0, Ordering::Relaxed);
        let ccache_cb_calls = CCACHE_CB_CALLS.swap(0, Ordering::Relaxed);
        let ccache_cb_touches = CCACHE_CB_TOUCHES.swap(0, Ordering::Relaxed);
        let ray_calls = RAY_CALLS.swap(0, Ordering::Relaxed);
        let ray_hits = RAY_HITS.swap(0, Ordering::Relaxed);
        let dray_calls = DOWN_RAY_CALLS.swap(0, Ordering::Relaxed);
        let dray_hits = DOWN_RAY_HITS.swap(0, Ordering::Relaxed);
        let sweep_calls = SWEEP_CALLS.swap(0, Ordering::Relaxed);
        let sweep_hits = SWEEP_HITS.swap(0, Ordering::Relaxed);
        let overlap_calls = OVERLAP_CALLS.swap(0, Ordering::Relaxed);
        let overlap_hits = OVERLAP_HITS.swap(0, Ordering::Relaxed);
        let fp_calls = FILTER_PRE_CALLS.swap(0, Ordering::Relaxed);
        let fp_nz = FILTER_PRE_NZ.swap(0, Ordering::Relaxed);
        let fpost_calls = FILTER_POST_CALLS.swap(0, Ordering::Relaxed);
        let ccfp_calls = CC_FILTER_PRE_CALLS.swap(0, Ordering::Relaxed);
        let ccfp_nz = CC_FILTER_PRE_NZ.swap(0, Ordering::Relaxed);
        let sdf_calls = SDF_CALLS.swap(0, Ordering::Relaxed);
        let caps_calls = CAPS_CALLS.swap(0, Ordering::Relaxed);

        let (cam_ptr, cam_pos, cam_dir) = {
            let live_cp = unsafe {
                let slot = exe::ptr::<usize>(CAM_STATE_PTR_VA) as usize;
                if in_pe_image(slot) {
                    exe::rd::<usize>(slot)
                } else {
                    0
                }
            };
            if live_cp != 0 {
                unsafe {
                    let dir = [
                        exe::rd::<f64>(live_cp + 0xa0),
                        exe::rd::<f64>(live_cp + 0xa8),
                        exe::rd::<f64>(live_cp + 0xb0),
                    ];
                    let pos = [
                        exe::rd::<f64>(live_cp + 0xb8),
                        exe::rd::<f64>(live_cp + 0xc0),
                        exe::rd::<f64>(live_cp + 0xc8),
                    ];
                    (live_cp, pos, dir)
                }
            } else {
                let cp = LAST_CAM_PTR.load(Ordering::Relaxed);
                if cp != 0 {
                    (
                        cp,
                        [
                            f64::from_bits(LAST_CAM_PX.load(Ordering::Relaxed)),
                            f64::from_bits(LAST_CAM_PY.load(Ordering::Relaxed)),
                            f64::from_bits(LAST_CAM_PZ.load(Ordering::Relaxed)),
                        ],
                        [
                            f64::from_bits(LAST_CAM_DX.load(Ordering::Relaxed)),
                            f64::from_bits(LAST_CAM_DY.load(Ordering::Relaxed)),
                            f64::from_bits(LAST_CAM_DZ.load(Ordering::Relaxed)),
                        ],
                    )
                } else {
                    (0, [0.0; 3], [0.0; 3])
                }
            }
        };

        // Drive optional W-hold walk test and detect gameplay transition.
        let mut holding_w = false;
        let mut cam_y_jump = false;
        if let Some(st_m) = WALK_ST.get() {
            if let Ok(mut st) = st_m.lock() {
                if !st.last_cam_y.is_nan() && (cam_pos[1] - st.last_cam_y).abs() > 0.25 {
                    cam_y_jump = true;
                }
                if cam_ptr != 0 {
                    st.last_cam_y = cam_pos[1];
                }
                if cc_calls > 0 {
                    st.active_cc_frames = st.active_cc_frames.saturating_add(1);
                    if !st.gameplay_detected && st.active_cc_frames >= 15 {
                        st.gameplay_detected = true;
                        emit_line(&format!(
                            "gameplay_detected frame={frame} cam_pos=({:.3},{:.3},{:.3}) cc_total={}",
                            cam_pos[0],
                            cam_pos[1],
                            cam_pos[2],
                            CC_TOTAL_CALLS.load(Ordering::Relaxed)
                        ));
                    }
                }
                if let Some((s_start, s_end)) = cfg.skip {
                    let want_skip = frame >= s_start && frame < s_end;
                    if frame >= s_start.saturating_sub(10) {
                        WALK_OVERRIDES_AUTOPLAY.store(true, Ordering::Relaxed);
                    }
                    if want_skip != st.skip_down {
                        w::user32::input::force_focus_for_autoplay(hwnd);
                        w::user32::input::native_key(w::user32::input::NativeKey {
                            mac: 0x31, // VK_SPACE
                            down: want_skip,
                            repeat: false,
                            chars: if want_skip { vec![0x20] } else { Vec::new() },
                        });
                        emit_line(&format!(
                            "skip_input frame={frame} SPACE={}",
                            if want_skip { "DOWN" } else { "UP" }
                        ));
                        st.skip_down = want_skip;
                    }
                }
                let want_w = match cfg.walk {
                    WalkMode::None => false,
                    WalkMode::Frames { start, end } => {
                        if frame >= start.saturating_sub(30)
                            && !WALK_OVERRIDES_AUTOPLAY.swap(true, Ordering::Relaxed)
                        {
                            w::user32::input::native_key(w::user32::input::NativeKey {
                                mac: 0x24,
                                down: false,
                                repeat: false,
                                chars: Vec::new(),
                            });
                            w::user32::input::native_key(w::user32::input::NativeKey {
                                mac: 0x31,
                                down: false,
                                repeat: false,
                                chars: Vec::new(),
                            });
                            w::misc::xinput1_4::backend_update(
                                0,
                                w::misc::xinput1_4::Gamepad::default(),
                            );
                        }
                        frame >= start && frame < end
                    }
                    WalkMode::Auto { delay, dur } => {
                        if st.active_cc_frames >= delay {
                            if !WALK_OVERRIDES_AUTOPLAY.swap(true, Ordering::Relaxed) {
                                w::user32::input::native_key(w::user32::input::NativeKey {
                                    mac: 0x24,
                                    down: false,
                                    repeat: false,
                                    chars: Vec::new(),
                                });
                                w::user32::input::native_key(w::user32::input::NativeKey {
                                    mac: 0x31,
                                    down: false,
                                    repeat: false,
                                    chars: Vec::new(),
                                });
                                w::misc::xinput1_4::backend_update(
                                    0,
                                    w::misc::xinput1_4::Gamepad::default(),
                                );
                            }
                            let s = *st.started_frame.get_or_insert(frame);
                            frame >= s && frame < s.saturating_add(dur)
                        } else {
                            false
                        }
                    }
                };
                if want_w != st.w_down {
                    w::user32::input::force_focus_for_autoplay(hwnd);
                    w::user32::input::native_key(w::user32::input::NativeKey {
                        mac: 0x0D, // VK_W
                        down: want_w,
                        repeat: false,
                        chars: if want_w {
                            vec![b'w' as u16]
                        } else {
                            Vec::new()
                        },
                    });
                    emit_line(&format!(
                        "walk_input frame={frame} W={}",
                        if want_w { "DOWN" } else { "UP" }
                    ));
                    st.w_down = want_w;
                }
                holding_w = st.w_down;
            }
        }

        let active = cc_calls > 0 || ray_calls > 0 || ccache_calls > 0;
        let should_log = cfg.verbose
            || cam_y_jump
            || (holding_w && frame % 2 == 0)
            || (active && frame % 15 == 0)
            || frame % 120 == 0
            || frame == 1;

        if should_log {
            let pin = [
                f32::from_bits(LAST_POS_IN_X.load(Ordering::Relaxed)),
                f32::from_bits(LAST_POS_IN_Y.load(Ordering::Relaxed)),
                f32::from_bits(LAST_POS_IN_Z.load(Ordering::Relaxed)),
            ];
            let pout = [
                f32::from_bits(LAST_POS_OUT_X.load(Ordering::Relaxed)),
                f32::from_bits(LAST_POS_OUT_Y.load(Ordering::Relaxed)),
                f32::from_bits(LAST_POS_OUT_Z.load(Ordering::Relaxed)),
            ];
            let vin = [
                f32::from_bits(LAST_VEL_IN_X.load(Ordering::Relaxed)),
                f32::from_bits(LAST_VEL_IN_Y.load(Ordering::Relaxed)),
                f32::from_bits(LAST_VEL_IN_Z.load(Ordering::Relaxed)),
            ];
            let vout = [
                f32::from_bits(LAST_VEL_OUT_X.load(Ordering::Relaxed)),
                f32::from_bits(LAST_VEL_OUT_Y.load(Ordering::Relaxed)),
                f32::from_bits(LAST_VEL_OUT_Z.load(Ordering::Relaxed)),
            ];
            let blk = LAST_CC_BLOCKED.load(Ordering::Relaxed);
            let on_g = LAST_CC_ON_GROUND.load(Ordering::Relaxed);
            let st = LAST_CC_STATUS.load(Ordering::Relaxed);
            let p9 = LAST_CC_P9.load(Ordering::Relaxed);
            let dt = f32::from_bits(LAST_CC_DT.load(Ordering::Relaxed));
            let cc_ptr = LAST_CC_PTR.load(Ordering::Relaxed);
            let px_scene = LAST_PX_SCENE.load(Ordering::Relaxed);
            let sdf_b60 = LAST_SDF_B60.load(Ordering::Relaxed);
            let caps_b60 = LAST_CAPS_B60.load(Ordering::Relaxed);
            let dray_sy = f32::from_bits(LAST_DRAY_SY.load(Ordering::Relaxed));
            let dray_ey = f32::from_bits(LAST_DRAY_EY.load(Ordering::Relaxed));
            let dray_hy = f32::from_bits(LAST_DRAY_HY.load(Ordering::Relaxed));
            let dray_hit = LAST_DRAY_HIT.load(Ordering::Relaxed);

            emit_line(&format!(
                "f={frame} W={} cam_p=({:.3},{:.3},{:.3}) cam_d=({:.3},{:.3},{:.3}) \
                 cc={cc_calls}(p9={cc_p9},p0={cc_p9z}) ptr={cc_ptr:#x} scene={px_scene:#x} \
                 pin=({:.3},{:.3},{:.3}) pout=({:.3},{:.3},{:.3}) \
                 vin=({:.3},{:.3},{:.3}) vout=({:.3},{:.3},{:.3}) \
                 on_g={on_g} blk={blk} st={st:#x} p9={p9} dt={dt:.4} sdf={sdf_calls}(b60={sdf_b60}) caps={caps_calls}(b60={caps_b60}) \
                 ccache={ccache_calls}(ne={ccache_ne},p={ccache_prims},m={ccache_meshes},t={ccache_tris},tmax={ccache_tris_max},cb={ccache_cb_calls}/{ccache_cb_touches}) \
                 ray={ray_hits}/{ray_calls} dray={dray_hits}/{dray_calls}(sy={dray_sy:.2},ey={dray_ey:.2},hy={dray_hy:.2},h={dray_hit}) \
                 swp={sweep_hits}/{sweep_calls} ovl={overlap_hits}/{overlap_calls} \
                 flt=({fp_nz}/{fp_calls},{fpost_calls},cc={ccfp_nz}/{ccfp_calls})",
                if holding_w { 1 } else { 0 },
                cam_pos[0],
                cam_pos[1],
                cam_pos[2],
                cam_dir[0],
                cam_dir[1],
                cam_dir[2],
                pin[0],
                pin[1],
                pin[2],
                pout[0],
                pout[1],
                pout[2],
                vin[0],
                vin[1],
                vin[2],
                vout[0],
                vout[1],
                vout[2],
            ));
        }

        if let Some(limit) = cfg.exit_at {
            if frame >= limit {
                emit_line(&format!("exit_at reached frame={frame}"));
                std::process::exit(0);
            }
        }
    }
}
