//! Native `gameinput.dll` (Microsoft GDK GameInput API) implementation for Apple platforms.
//!
//! Modern Microsoft GDK / Xbox Game Studios titles, Unreal Engine 5.4+, Halo Infinite,
//! Forza Horizon 5 / Forza Motorsport, Starfield, Sea of Thieves, and Call of Duty
//! use `GameInputCreate` (`IGameInput`) for unified low-latency keyboard, mouse, and
//! controller input with impulse triggers.
//!
//! Backed directly by live `win32-rt` input subsystems:
//! * Gamepads (up to 4 slots): `crate::misc::xinput1_4` + `crate::platform::gamepad`
//!   (GameController.framework + CoreHaptics rumble, plus iOS virtual overlay/Taptic Engine).
//! * Keyboard: `crate::user32::input::GetAsyncKeyState` + `crate::user32::keymap`.
//! * Mouse: `crate::user32::cursor::cursor_pos` + `crate::user32::input::GetAsyncKeyState`.

#![allow(non_snake_case)]

use crate::misc::xinput1_4;
use crate::user32::keymap;
use std::collections::{HashMap, VecDeque};
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU32, Ordering};
use std::sync::{Mutex, Once, OnceLock};
use std::time::Instant;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// ---------------------------------------------------------------------------
// HRESULTs & GameInput ABI constants (Microsoft GDK `GameInput.h`)
// ---------------------------------------------------------------------------

pub const S_OK: i32 = 0;
pub const E_NOTIMPL: i32 = 0x8000_4001u32 as i32;
pub const E_NOINTERFACE: i32 = 0x8000_4002u32 as i32;
pub const E_POINTER: i32 = 0x8000_4003u32 as i32;
pub const E_INVALIDARG: i32 = 0x8007_0057u32 as i32;

pub const GAMEINPUT_E_DEVICE_DISCONNECTED: i32 = 0x838A_0001u32 as i32;
pub const GAMEINPUT_E_DEVICE_NOT_FOUND: i32 = 0x838A_0002u32 as i32;
pub const GAMEINPUT_E_READING_NOT_FOUND: i32 = 0x838A_0003u32 as i32;
pub const GAMEINPUT_E_REFERENCE_READING_TOO_OLD: i32 = 0x838A_0004u32 as i32;

// `GameInputKind` bitflags (exact Windows / Microsoft GDK `GameInput.h` values)
pub const GAME_INPUT_KIND_UNKNOWN: u32 = 0x0000_0000;
pub const GAME_INPUT_KIND_RAW_DEVICE_REPORT: u32 = 0x0000_0001;
pub const GAME_INPUT_KIND_CONTROLLER_AXIS: u32 = 0x0000_0002;
pub const GAME_INPUT_KIND_CONTROLLER_BUTTON: u32 = 0x0000_0004;
pub const GAME_INPUT_KIND_CONTROLLER_SWITCH: u32 = 0x0000_0008;
pub const GAME_INPUT_KIND_CONTROLLER: u32 = 0x0000_000E;
pub const GAME_INPUT_KIND_KEYBOARD: u32 = 0x0000_0010;
pub const GAME_INPUT_KIND_MOUSE: u32 = 0x0000_0020;
pub const GAME_INPUT_KIND_TOUCH: u32 = 0x0000_0100;
pub const GAME_INPUT_KIND_MOTION: u32 = 0x0000_1000;
pub const GAME_INPUT_KIND_ARCADE_STICK: u32 = 0x0001_0000;
pub const GAME_INPUT_KIND_FLIGHT_STICK: u32 = 0x0002_0000;
pub const GAME_INPUT_KIND_GAMEPAD: u32 = 0x0004_0000;
pub const GAME_INPUT_KIND_RACING_WHEEL: u32 = 0x0008_0000;
pub const GAME_INPUT_KIND_UI_NAVIGATION: u32 = 0x0100_0000;

// `GameInputGamepadButtons` bitflags
pub const GAME_INPUT_GAMEPAD_NONE: u32 = 0x0000_0000;
pub const GAME_INPUT_GAMEPAD_MENU: u32 = 0x0000_0001;
pub const GAME_INPUT_GAMEPAD_VIEW: u32 = 0x0000_0002;
pub const GAME_INPUT_GAMEPAD_A: u32 = 0x0000_0004;
pub const GAME_INPUT_GAMEPAD_B: u32 = 0x0000_0008;
pub const GAME_INPUT_GAMEPAD_X: u32 = 0x0000_0010;
pub const GAME_INPUT_GAMEPAD_Y: u32 = 0x0000_0020;
pub const GAME_INPUT_GAMEPAD_DPAD_UP: u32 = 0x0000_0040;
pub const GAME_INPUT_GAMEPAD_DPAD_DOWN: u32 = 0x0000_0080;
pub const GAME_INPUT_GAMEPAD_DPAD_LEFT: u32 = 0x0000_0100;
pub const GAME_INPUT_GAMEPAD_DPAD_RIGHT: u32 = 0x0000_0200;
pub const GAME_INPUT_GAMEPAD_LEFT_SHOULDER: u32 = 0x0000_0400;
pub const GAME_INPUT_GAMEPAD_RIGHT_SHOULDER: u32 = 0x0000_0800;
pub const GAME_INPUT_GAMEPAD_LEFT_THUMBSTICK: u32 = 0x0000_1000;
pub const GAME_INPUT_GAMEPAD_RIGHT_THUMBSTICK: u32 = 0x0000_2000;

const GAMEPAD_STANDARD_LAYOUT: u32 = GAME_INPUT_GAMEPAD_MENU
    | GAME_INPUT_GAMEPAD_VIEW
    | GAME_INPUT_GAMEPAD_A
    | GAME_INPUT_GAMEPAD_B
    | GAME_INPUT_GAMEPAD_X
    | GAME_INPUT_GAMEPAD_Y
    | GAME_INPUT_GAMEPAD_DPAD_UP
    | GAME_INPUT_GAMEPAD_DPAD_DOWN
    | GAME_INPUT_GAMEPAD_DPAD_LEFT
    | GAME_INPUT_GAMEPAD_DPAD_RIGHT
    | GAME_INPUT_GAMEPAD_LEFT_SHOULDER
    | GAME_INPUT_GAMEPAD_RIGHT_SHOULDER
    | GAME_INPUT_GAMEPAD_LEFT_THUMBSTICK
    | GAME_INPUT_GAMEPAD_RIGHT_THUMBSTICK;

// `GameInputMouseButtons` bitflags
pub const GAME_INPUT_MOUSE_NONE: u32 = 0x0000_0000;
pub const GAME_INPUT_MOUSE_LEFT_BUTTON: u32 = 0x0000_0001;
pub const GAME_INPUT_MOUSE_RIGHT_BUTTON: u32 = 0x0000_0002;
pub const GAME_INPUT_MOUSE_MIDDLE_BUTTON: u32 = 0x0000_0004;
pub const GAME_INPUT_MOUSE_BUTTON_4: u32 = 0x0000_0008;
pub const GAME_INPUT_MOUSE_BUTTON_5: u32 = 0x0000_0010;
pub const GAME_INPUT_MOUSE_WHEEL_TILT_LEFT: u32 = 0x0000_0020;
pub const GAME_INPUT_MOUSE_WHEEL_TILT_RIGHT: u32 = 0x0000_0040;

// `GameInputDeviceStatus` bitflags
pub const GAME_INPUT_DEVICE_NO_STATUS: u32 = 0x0000_0000;
pub const GAME_INPUT_DEVICE_CONNECTED: u32 = 0x0000_0001;
pub const GAME_INPUT_DEVICE_INPUT_ENABLED: u32 = 0x0000_0002;
pub const GAME_INPUT_DEVICE_OUTPUT_ENABLED: u32 = 0x0000_0004;
pub const GAME_INPUT_DEVICE_ANY_STATUS: u32 = 0x00FF_FFFF;

// `GameInputDeviceFamily` enum
pub const GAME_INPUT_FAMILY_VIRTUAL: i32 = -1;
pub const GAME_INPUT_FAMILY_AGGREGATE: i32 = 0;
pub const GAME_INPUT_FAMILY_XBOX_ONE: i32 = 1;
pub const GAME_INPUT_FAMILY_XBOX_360: i32 = 2;
pub const GAME_INPUT_FAMILY_HID: i32 = 3;
pub const GAME_INPUT_FAMILY_I8042: i32 = 4;

// `GameInputEnumerationKind` enum
pub const GAME_INPUT_NO_ENUMERATION: u32 = 0;
pub const GAME_INPUT_ASYNC_ENUMERATION: u32 = 1;
pub const GAME_INPUT_BLOCKING_ENUMERATION: u32 = 2;

// `GameInputRumbleMotors` bitflags
pub const GAME_INPUT_RUMBLE_NONE: u32 = 0x0000_0000;
pub const GAME_INPUT_RUMBLE_LOW_FREQUENCY: u32 = 0x0000_0001;
pub const GAME_INPUT_RUMBLE_HIGH_FREQUENCY: u32 = 0x0000_0002;
pub const GAME_INPUT_RUMBLE_LEFT_TRIGGER: u32 = 0x0000_0004;
pub const GAME_INPUT_RUMBLE_RIGHT_TRIGGER: u32 = 0x0000_0008;

// `GameInputBatteryStatus` enum
pub const GAME_INPUT_BATTERY_UNKNOWN: i32 = -1;
pub const GAME_INPUT_BATTERY_NOT_PRESENT: i32 = 0;
pub const GAME_INPUT_BATTERY_DISCHARGING: i32 = 1;
pub const GAME_INPUT_BATTERY_IDLE: i32 = 2;
pub const GAME_INPUT_BATTERY_CHARGING: i32 = 3;

// `GameInputSwitchPosition` enum
pub const GAME_INPUT_SWITCH_CENTER: u32 = 0;
pub const GAME_INPUT_SWITCH_UP: u32 = 1;
pub const GAME_INPUT_SWITCH_UP_RIGHT: u32 = 2;
pub const GAME_INPUT_SWITCH_RIGHT: u32 = 3;
pub const GAME_INPUT_SWITCH_DOWN_RIGHT: u32 = 4;
pub const GAME_INPUT_SWITCH_DOWN: u32 = 5;
pub const GAME_INPUT_SWITCH_DOWN_LEFT: u32 = 6;
pub const GAME_INPUT_SWITCH_LEFT: u32 = 7;
pub const GAME_INPUT_SWITCH_UP_LEFT: u32 = 8;

// `GameInputKeyboardKind` enum
pub const GAME_INPUT_ANSI_KEYBOARD: i32 = 0;

// `GameInputLabel` enum values for Xbox controller layout
const LABEL_XBOX_MENU: i32 = 4;
const LABEL_XBOX_VIEW: i32 = 5;
const LABEL_XBOX_A: i32 = 7;
const LABEL_XBOX_B: i32 = 8;
const LABEL_XBOX_X: i32 = 9;
const LABEL_XBOX_Y: i32 = 10;
const LABEL_XBOX_DPAD_UP: i32 = 11;
const LABEL_XBOX_DPAD_DOWN: i32 = 12;
const LABEL_XBOX_DPAD_LEFT: i32 = 13;
const LABEL_XBOX_DPAD_RIGHT: i32 = 14;
const LABEL_XBOX_LEFT_SHOULDER: i32 = 15;
const LABEL_XBOX_RIGHT_SHOULDER: i32 = 16;
const LABEL_XBOX_LEFT_STICK_BTN: i32 = 19;
const LABEL_XBOX_RIGHT_STICK_BTN: i32 = 20;

// COM Interface GUIDs
type Guid = [u8; 16];

const fn guid(d1: u32, d2: u16, d3: u16, d4: [u8; 8]) -> Guid {
    let a = d1.to_le_bytes();
    let b = d2.to_le_bytes();
    let c = d3.to_le_bytes();
    [a[0], a[1], a[2], a[3], b[0], b[1], c[0], c[1], d4[0], d4[1], d4[2], d4[3], d4[4], d4[5], d4[6], d4[7]]
}

const IID_IUNKNOWN: Guid = guid(0x0000_0000, 0x0000, 0x0000, [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46]);
const IID_IGAMEINPUT: Guid = guid(0x11BE_2A7E, 0x4254, 0x445A, [0x9C, 0x09, 0xFF, 0xC4, 0x0F, 0x00, 0x69, 0x18]);
const IID_IGAMEINPUT_READING: Guid = guid(0x2156_947A, 0xE1FA, 0x4DE0, [0xA3, 0x0B, 0xD8, 0x12, 0x93, 0x1D, 0xBD, 0x8D]);
const IID_IGAMEINPUT_DEVICE: Guid = guid(0x31DD_86FB, 0x4C1B, 0x408A, [0x86, 0x8F, 0x43, 0x9B, 0x3C, 0xD4, 0x71, 0x25]);
const IID_IGAMEINPUT_DISPATCHER: Guid = guid(0x415E_ED2E, 0x98CB, 0x42C2, [0x8F, 0x28, 0xB9, 0x46, 0x01, 0x07, 0x4E, 0x31]);

// ---------------------------------------------------------------------------
// Win64 ABI structs (`GameInput.h`)
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GameInputUsage {
    pub page: u16,
    pub id: u16,
}
const _: () = assert!(std::mem::size_of::<GameInputUsage>() == 4);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GameInputVersion {
    pub major: u16,
    pub minor: u16,
    pub build: u16,
    pub revision: u16,
}
const _: () = assert!(std::mem::size_of::<GameInputVersion>() == 8);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AppLocalDeviceId {
    pub value: [u8; 32],
}
const _: () = assert!(std::mem::size_of::<AppLocalDeviceId>() == 32);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct GameInputString {
    pub size_in_bytes: u32,
    pub code_point_count: u32,
    pub data: *const u8,
}
unsafe impl Sync for GameInputString {}
unsafe impl Send for GameInputString {}
const _: () = assert!(std::mem::size_of::<GameInputString>() == 16);

const fn game_input_str(nul_terminated_ascii: &'static [u8]) -> GameInputString {
    let len = if nul_terminated_ascii.is_empty() {
        0
    } else {
        nul_terminated_ascii.len() - 1
    };
    GameInputString {
        size_in_bytes: len as u32,
        code_point_count: len as u32,
        data: nul_terminated_ascii.as_ptr(),
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GameInputKeyboardInfo {
    pub kind: i32,
    pub layout: u32,
    pub key_count: u32,
    pub function_key_count: u32,
    pub max_simultaneous_keys: u32,
    pub platform_type: u32,
    pub platform_subtype: u32,
}
const _: () = assert!(std::mem::size_of::<GameInputKeyboardInfo>() == 28);

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GameInputMouseInfo {
    pub supported_buttons: u32,
    pub sample_rate: u32,
    pub sensor_dpi: u32,
    pub has_wheel_x: u8,
    pub has_wheel_y: u8,
    pub _pad: [u8; 2],
}
const _: () = assert!(std::mem::size_of::<GameInputMouseInfo>() == 16);

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GameInputGamepadInfo {
    pub supported_layout: u32,
    pub menu_button_label: i32,
    pub view_button_label: i32,
    pub a_button_label: i32,
    pub b_button_label: i32,
    pub x_button_label: i32,
    pub y_button_label: i32,
    pub dpad_up_label: i32,
    pub dpad_down_label: i32,
    pub dpad_left_label: i32,
    pub dpad_right_label: i32,
    pub left_shoulder_button_label: i32,
    pub right_shoulder_button_label: i32,
    pub left_thumbstick_button_label: i32,
    pub right_thumbstick_button_label: i32,
    pub extra_button_count: u32,
    pub extra_axis_count: u32,
}
const _: () = assert!(std::mem::size_of::<GameInputGamepadInfo>() == 68);

#[repr(C)]
pub struct GameInputDeviceInfo {
    pub info_size: u32,
    pub vendor_id: u16,
    pub product_id: u16,
    pub revision_number: u16,
    pub interface_number: u8,
    pub collection_number: u8,
    pub usage: GameInputUsage,
    pub hardware_version: GameInputVersion,
    pub firmware_version: GameInputVersion,
    pub device_id: AppLocalDeviceId,
    pub device_root_id: AppLocalDeviceId,
    pub device_family: i32,
    pub capabilities: u32,
    pub supported_input: u32,
    pub supported_rumble_motors: u32,
    pub input_report_count: u32,
    pub output_report_count: u32,
    pub feature_report_count: u32,
    pub controller_axis_count: u32,
    pub controller_button_count: u32,
    pub controller_switch_count: u32,
    pub touch_point_count: u32,
    pub touch_sensor_count: u32,
    pub force_feedback_motor_count: u32,
    pub haptic_feedback_motor_count: u32,
    pub device_string_count: u32,
    pub device_descriptor_size: u32,
    pub input_report_info: *const c_void,
    pub output_report_info: *const c_void,
    pub feature_report_info: *const c_void,
    pub controller_axis_info: *const c_void,
    pub controller_button_info: *const c_void,
    pub controller_switch_info: *const c_void,
    pub keyboard_info: *const GameInputKeyboardInfo,
    pub mouse_info: *const GameInputMouseInfo,
    pub touch_sensor_info: *const c_void,
    pub motion_info: *const c_void,
    pub arcade_stick_info: *const c_void,
    pub flight_stick_info: *const c_void,
    pub gamepad_info: *const GameInputGamepadInfo,
    pub racing_wheel_info: *const c_void,
    pub force_feedback_motor_info: *const c_void,
    pub haptic_feedback_motor_info: *const c_void,
    pub display_name: *const GameInputString,
    pub device_strings: *const GameInputString,
    pub device_descriptor_data: *const c_void,
    pub supported_system_buttons: u32,
    pub _pad_end: u32,
}
unsafe impl Sync for GameInputDeviceInfo {}
unsafe impl Send for GameInputDeviceInfo {}
const _: () = assert!(std::mem::size_of::<GameInputDeviceInfo>() == 320);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputGamepadState {
    pub buttons: u32,
    pub left_trigger: f32,
    pub right_trigger: f32,
    pub left_thumbstick_x: f32,
    pub left_thumbstick_y: f32,
    pub right_thumbstick_x: f32,
    pub right_thumbstick_y: f32,
}
const _: () = assert!(std::mem::size_of::<GameInputGamepadState>() == 28);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GameInputKeyState {
    pub scan_code: u32,
    pub code_point: u32,
    pub virtual_key: u8,
    pub is_dead_key: u8,
    pub _pad: [u8; 2],
}
const _: () = assert!(std::mem::size_of::<GameInputKeyState>() == 12);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GameInputMouseState {
    pub buttons: u32,
    pub _pad: u32,
    pub position_x: i64,
    pub position_y: i64,
    pub wheel_x: i64,
    pub wheel_y: i64,
}
const _: () = assert!(std::mem::size_of::<GameInputMouseState>() == 40);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputRumbleParams {
    pub low_frequency: f32,
    pub high_frequency: f32,
    pub left_trigger: f32,
    pub right_trigger: f32,
}
const _: () = assert!(std::mem::size_of::<GameInputRumbleParams>() == 16);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputBatteryState {
    pub charge_rate: f32,
    pub max_charge_rate: f32,
    pub remaining_capacity: f32,
    pub full_charge_capacity: f32,
    pub status: i32,
}
const _: () = assert!(std::mem::size_of::<GameInputBatteryState>() == 20);

// ---------------------------------------------------------------------------
// Static device metadata
// ---------------------------------------------------------------------------

static KEYBOARD_INFO: GameInputKeyboardInfo = GameInputKeyboardInfo {
    kind: GAME_INPUT_ANSI_KEYBOARD,
    layout: 0x0409_0409,
    key_count: 104,
    function_key_count: 12,
    max_simultaneous_keys: 16,
    platform_type: 4,
    platform_subtype: 0,
};

static MOUSE_INFO: GameInputMouseInfo = GameInputMouseInfo {
    supported_buttons: GAME_INPUT_MOUSE_LEFT_BUTTON
        | GAME_INPUT_MOUSE_RIGHT_BUTTON
        | GAME_INPUT_MOUSE_MIDDLE_BUTTON
        | GAME_INPUT_MOUSE_BUTTON_4
        | GAME_INPUT_MOUSE_BUTTON_5,
    sample_rate: 1000,
    sensor_dpi: 800,
    has_wheel_x: 1,
    has_wheel_y: 1,
    _pad: [0; 2],
};

static GAMEPAD_INFO: GameInputGamepadInfo = GameInputGamepadInfo {
    supported_layout: GAMEPAD_STANDARD_LAYOUT,
    menu_button_label: LABEL_XBOX_MENU,
    view_button_label: LABEL_XBOX_VIEW,
    a_button_label: LABEL_XBOX_A,
    b_button_label: LABEL_XBOX_B,
    x_button_label: LABEL_XBOX_X,
    y_button_label: LABEL_XBOX_Y,
    dpad_up_label: LABEL_XBOX_DPAD_UP,
    dpad_down_label: LABEL_XBOX_DPAD_DOWN,
    dpad_left_label: LABEL_XBOX_DPAD_LEFT,
    dpad_right_label: LABEL_XBOX_DPAD_RIGHT,
    left_shoulder_button_label: LABEL_XBOX_LEFT_SHOULDER,
    right_shoulder_button_label: LABEL_XBOX_RIGHT_SHOULDER,
    left_thumbstick_button_label: LABEL_XBOX_LEFT_STICK_BTN,
    right_thumbstick_button_label: LABEL_XBOX_RIGHT_STICK_BTN,
    extra_button_count: 0,
    extra_axis_count: 0,
};

static NAME_KEYBOARD: GameInputString = game_input_str(b"Keyboard\0");
static NAME_MOUSE: GameInputString = game_input_str(b"Mouse\0");
static NAME_GAMEPAD: [GameInputString; xinput1_4::XUSER_MAX_COUNT] = [
    game_input_str(b"Gamepad 1\0"),
    game_input_str(b"Gamepad 2\0"),
    game_input_str(b"Gamepad 3\0"),
    game_input_str(b"Gamepad 4\0"),
];

const fn make_device_id(tag: u8, slot: u8) -> AppLocalDeviceId {
    let mut value = [0u8; 32];
    value[0] = b'G';
    value[1] = b'I';
    value[2] = b'N';
    value[3] = b'P';
    value[4] = tag;
    value[5] = slot;
    AppLocalDeviceId { value }
}

// ---------------------------------------------------------------------------
// Input state conversion & live snapshot helpers
// ---------------------------------------------------------------------------

static GAMEPAD_STARTED: AtomicBool = AtomicBool::new(false);
static MOUSE_WHEEL_X: AtomicI64 = AtomicI64::new(0);
static MOUSE_WHEEL_Y: AtomicI64 = AtomicI64::new(0);

fn ensure_gamepad_started() {
    if !GAMEPAD_STARTED.swap(true, Ordering::AcqRel) {
        crate::platform::gamepad::start();
    }
}

/// Accumulate horizontal/vertical scroll wheel ticks into the GameInput mouse stream.
pub fn record_mouse_wheel(dx: i64, dy: i64) {
    if dx != 0 {
        MOUSE_WHEEL_X.fetch_add(dx, Ordering::Relaxed);
    }
    if dy != 0 {
        MOUSE_WHEEL_Y.fetch_add(dy, Ordering::Relaxed);
    }
    if dx != 0 || dy != 0 {
        on_mouse_changed();
    }
}

static GAMEINPUT_ACTIVE: AtomicBool = AtomicBool::new(false);

/// Capture a gamepad's initial reading at connection time so historical
/// `GetTemporalReading(t0, ...)` lookups succeed even if input transitions arrive
/// before the caller's first reading query.
pub(crate) fn on_gamepad_connected(slot: usize, initial_pad: &xinput1_4::Gamepad) {
    if slot >= xinput1_4::XUSER_MAX_COUNT {
        return;
    }
    let ts = current_timestamp_us();
    let payload = ReadingPayload::Gamepad(xinput_to_gamepad_state(initial_pad));
    let mut st = runtime_state().lock().unwrap();
    st.set_baseline(DeviceKind::Gamepad(slot), GAME_INPUT_KIND_GAMEPAD, payload, ts);
}

pub(crate) fn on_gamepad_disconnected(slot: usize) {
    if slot >= xinput1_4::XUSER_MAX_COUNT {
        return;
    }
    runtime_state().lock().unwrap().clear_baseline(DeviceKind::Gamepad(slot));
}

/// Record a gamepad input transition with its event timestamp immediately when
/// the backend updates so subsequent `GetCurrentTimestamp()` → `GetTemporalReading()`
/// queries find the transition without requiring a future timestamp offset.
pub(crate) fn on_gamepad_state_changed(slot: usize, pad: &xinput1_4::Gamepad) {
    if slot >= xinput1_4::XUSER_MAX_COUNT || !GAMEINPUT_ACTIVE.load(Ordering::Acquire) {
        return;
    }
    let ts = current_timestamp_us();
    let payload = ReadingPayload::Gamepad(xinput_to_gamepad_state(pad));
    let mut st = runtime_state().lock().unwrap();
    if st.baseline(DeviceKind::Gamepad(slot)).is_none() {
        st.set_baseline(DeviceKind::Gamepad(slot), GAME_INPUT_KIND_GAMEPAD, payload.clone(), ts);
    }
    let _ = st.record_or_reuse_at(DeviceKind::Gamepad(slot), GAME_INPUT_KIND_GAMEPAD, payload, ts);
}

/// Record a keyboard state transition with its event timestamp when `user32::input`
/// processes a key event.
pub(crate) fn on_keyboard_changed() {
    if !GAMEINPUT_ACTIVE.load(Ordering::Acquire) {
        return;
    }
    let ts = current_timestamp_us();
    let payload = ReadingPayload::Keyboard(snapshot_keyboard());
    let mut st = runtime_state().lock().unwrap();
    let _ = st.record_or_reuse_at(DeviceKind::Keyboard, GAME_INPUT_KIND_KEYBOARD, payload, ts);
}

/// Record a mouse state transition with its event timestamp when `user32::input`
/// processes a mouse move, button, or wheel event.
pub(crate) fn on_mouse_changed() {
    if !GAMEINPUT_ACTIVE.load(Ordering::Acquire) {
        return;
    }
    let ts = current_timestamp_us();
    let payload = ReadingPayload::Mouse(snapshot_mouse());
    let mut st = runtime_state().lock().unwrap();
    let _ = st.record_or_reuse_at(DeviceKind::Mouse, GAME_INPUT_KIND_MOUSE, payload, ts);
}

fn current_timestamp_us() -> u64 {
    static EPOCH: OnceLock<Instant> = OnceLock::new();
    let start = *EPOCH.get_or_init(Instant::now);
    start.elapsed().as_micros() as u64 + 1
}

fn normalize_thumb(v: i16) -> f32 {
    if v < 0 {
        (v as f32) / 32768.0
    } else {
        (v as f32) / 32767.0
    }
}

fn clamp_unit(v: f32) -> f32 {
    if v.is_finite() {
        v.clamp(0.0, 1.0)
    } else {
        0.0
    }
}

/// Translate an `XINPUT_GAMEPAD` snapshot into `GameInputGamepadState`.
pub fn xinput_to_gamepad_state(pad: &xinput1_4::Gamepad) -> GameInputGamepadState {
    let mut buttons = GAME_INPUT_GAMEPAD_NONE;
    let b = pad.buttons;
    if b & xinput1_4::btn::START != 0 {
        buttons |= GAME_INPUT_GAMEPAD_MENU;
    }
    if b & xinput1_4::btn::BACK != 0 {
        buttons |= GAME_INPUT_GAMEPAD_VIEW;
    }
    if b & xinput1_4::btn::A != 0 {
        buttons |= GAME_INPUT_GAMEPAD_A;
    }
    if b & xinput1_4::btn::B != 0 {
        buttons |= GAME_INPUT_GAMEPAD_B;
    }
    if b & xinput1_4::btn::X != 0 {
        buttons |= GAME_INPUT_GAMEPAD_X;
    }
    if b & xinput1_4::btn::Y != 0 {
        buttons |= GAME_INPUT_GAMEPAD_Y;
    }
    if b & xinput1_4::btn::DPAD_UP != 0 {
        buttons |= GAME_INPUT_GAMEPAD_DPAD_UP;
    }
    if b & xinput1_4::btn::DPAD_DOWN != 0 {
        buttons |= GAME_INPUT_GAMEPAD_DPAD_DOWN;
    }
    if b & xinput1_4::btn::DPAD_LEFT != 0 {
        buttons |= GAME_INPUT_GAMEPAD_DPAD_LEFT;
    }
    if b & xinput1_4::btn::DPAD_RIGHT != 0 {
        buttons |= GAME_INPUT_GAMEPAD_DPAD_RIGHT;
    }
    if b & xinput1_4::btn::LEFT_SHOULDER != 0 {
        buttons |= GAME_INPUT_GAMEPAD_LEFT_SHOULDER;
    }
    if b & xinput1_4::btn::RIGHT_SHOULDER != 0 {
        buttons |= GAME_INPUT_GAMEPAD_RIGHT_SHOULDER;
    }
    if b & xinput1_4::btn::LEFT_THUMB != 0 {
        buttons |= GAME_INPUT_GAMEPAD_LEFT_THUMBSTICK;
    }
    if b & xinput1_4::btn::RIGHT_THUMB != 0 {
        buttons |= GAME_INPUT_GAMEPAD_RIGHT_THUMBSTICK;
    }
    GameInputGamepadState {
        buttons,
        left_trigger: (pad.left_trigger as f32) / 255.0,
        right_trigger: (pad.right_trigger as f32) / 255.0,
        left_thumbstick_x: normalize_thumb(pad.thumb_lx),
        left_thumbstick_y: normalize_thumb(pad.thumb_ly),
        right_thumbstick_x: normalize_thumb(pad.thumb_rx),
        right_thumbstick_y: normalize_thumb(pad.thumb_ry),
    }
}

/// Convert `GameInputRumbleParams` (`0.0..=1.0`) to XInput motor speeds (`0..=65535`).
/// Impulse trigger intensities blend into their respective side motors when separate
/// trigger actuators are unavailable.
fn rumble_params_to_xinput(params: GameInputRumbleParams) -> (u16, u16) {
    let low = clamp_unit(params.low_frequency);
    let high = clamp_unit(params.high_frequency);
    let lt = clamp_unit(params.left_trigger);
    let rt = clamp_unit(params.right_trigger);
    let left = (low.max(lt) * 65535.0).round() as u16;
    let right = (high.max(rt) * 65535.0).round() as u16;
    (left, right)
}

fn dpad_to_switch(buttons: u32) -> u32 {
    let up = buttons & GAME_INPUT_GAMEPAD_DPAD_UP != 0;
    let down = buttons & GAME_INPUT_GAMEPAD_DPAD_DOWN != 0;
    let left = buttons & GAME_INPUT_GAMEPAD_DPAD_LEFT != 0;
    let right = buttons & GAME_INPUT_GAMEPAD_DPAD_RIGHT != 0;
    let u = up && !down;
    let d = down && !up;
    let l = left && !right;
    let r = right && !left;
    match (u, d, l, r) {
        (true, false, false, false) => GAME_INPUT_SWITCH_UP,
        (true, false, false, true) => GAME_INPUT_SWITCH_UP_RIGHT,
        (false, false, false, true) => GAME_INPUT_SWITCH_RIGHT,
        (false, true, false, true) => GAME_INPUT_SWITCH_DOWN_RIGHT,
        (false, true, false, false) => GAME_INPUT_SWITCH_DOWN,
        (false, true, true, false) => GAME_INPUT_SWITCH_DOWN_LEFT,
        (false, false, true, false) => GAME_INPUT_SWITCH_LEFT,
        (true, false, true, false) => GAME_INPUT_SWITCH_UP_LEFT,
        _ => GAME_INPUT_SWITCH_CENTER,
    }
}

fn snapshot_keyboard() -> Vec<GameInputKeyState> {
    let shift = crate::user32::input::shift_down();
    let caps = crate::user32::input::caps_on();
    let lshift = crate::user32::input::async_down(keymap::VK_LSHIFT);
    let rshift = crate::user32::input::async_down(keymap::VK_RSHIFT);
    let lctrl = crate::user32::input::async_down(keymap::VK_LCONTROL);
    let rctrl = crate::user32::input::async_down(keymap::VK_RCONTROL);
    let lalt = crate::user32::input::async_down(keymap::VK_LMENU);
    let ralt = crate::user32::input::async_down(keymap::VK_RMENU);

    let mut keys = Vec::new();
    for vk in 0x08u8..=0xFEu8 {
        if (vk == keymap::VK_SHIFT && (lshift || rshift))
            || (vk == keymap::VK_CONTROL && (lctrl || rctrl))
            || (vk == keymap::VK_MENU && (lalt || ralt))
        {
            continue;
        }
        let down = match vk {
            keymap::VK_LSHIFT => lshift,
            keymap::VK_RSHIFT => rshift,
            keymap::VK_LCONTROL => lctrl,
            keymap::VK_RCONTROL => rctrl,
            keymap::VK_LMENU => lalt,
            keymap::VK_RMENU => ralt,
            _ => crate::user32::input::async_down(vk),
        };
        if !down {
            continue;
        }
        let scan_code = keymap::map_virtual_key(vk as u32, keymap::MAPVK_VK_TO_VSC_EX);
        let code_point = keymap::us_char(vk as u32, shift, caps).map(|c| c as u32).unwrap_or(0);
        keys.push(GameInputKeyState {
            scan_code,
            code_point,
            virtual_key: vk,
            is_dead_key: 0,
            _pad: [0; 2],
        });
    }
    keys
}

fn snapshot_mouse() -> GameInputMouseState {
    let mut buttons = GAME_INPUT_MOUSE_NONE;
    if crate::user32::input::async_down(keymap::VK_LBUTTON) {
        buttons |= GAME_INPUT_MOUSE_LEFT_BUTTON;
    }
    if crate::user32::input::async_down(keymap::VK_RBUTTON) {
        buttons |= GAME_INPUT_MOUSE_RIGHT_BUTTON;
    }
    if crate::user32::input::async_down(keymap::VK_MBUTTON) {
        buttons |= GAME_INPUT_MOUSE_MIDDLE_BUTTON;
    }
    if crate::user32::input::async_down(keymap::VK_XBUTTON1) {
        buttons |= GAME_INPUT_MOUSE_BUTTON_4;
    }
    if crate::user32::input::async_down(keymap::VK_XBUTTON2) {
        buttons |= GAME_INPUT_MOUSE_BUTTON_5;
    }
    let pt = crate::user32::cursor::cursor_pos();
    GameInputMouseState {
        buttons,
        _pad: 0,
        position_x: pt.x as i64,
        position_y: pt.y as i64,
        wheel_x: MOUSE_WHEEL_X.load(Ordering::Relaxed),
        wheel_y: MOUSE_WHEEL_Y.load(Ordering::Relaxed),
    }
}

// ---------------------------------------------------------------------------
// Shared runtime state (ring history + callback registry + persistent devices)
// ---------------------------------------------------------------------------

const READING_HISTORY_CAP: usize = 128;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DeviceKind {
    Keyboard,
    Mouse,
    Gamepad(usize),
}

#[derive(Clone, Debug, PartialEq)]
enum ReadingPayload {
    Gamepad(GameInputGamepadState),
    Keyboard(Vec<GameInputKeyState>),
    Mouse(GameInputMouseState),
}

#[derive(Clone, Debug)]
struct ReadingSnapshot {
    history_id: u64,
    sequence_number: u64,
    timestamp_us: u64,
    input_kind: u32,
    device_kind: DeviceKind,
    payload: ReadingPayload,
}

#[derive(Clone, Copy, Debug)]
enum CallbackKind {
    Reading,
    Device,
    SystemButton,
    KeyboardLayout,
}

struct CallbackEntry {
    kind: CallbackKind,
    active: bool,
}

struct RuntimeState {
    history: VecDeque<ReadingSnapshot>,
    next_history_id: u64,
    seq_gamepad: [u64; xinput1_4::XUSER_MAX_COUNT],
    seq_keyboard: u64,
    seq_mouse: u64,
    baseline_gamepad: [Option<ReadingSnapshot>; xinput1_4::XUSER_MAX_COUNT],
    baseline_keyboard: Option<ReadingSnapshot>,
    baseline_mouse: Option<ReadingSnapshot>,
    next_callback_token: u64,
    callbacks: HashMap<u64, CallbackEntry>,
    focus_policy: u32,
}

impl RuntimeState {
    fn new() -> Self {
        Self {
            history: VecDeque::with_capacity(READING_HISTORY_CAP),
            next_history_id: 0,
            seq_gamepad: [0; xinput1_4::XUSER_MAX_COUNT],
            seq_keyboard: 0,
            seq_mouse: 0,
            baseline_gamepad: std::array::from_fn(|_| None),
            baseline_keyboard: None,
            baseline_mouse: None,
            next_callback_token: 1,
            callbacks: HashMap::new(),
            focus_policy: 0,
        }
    }

    fn next_seq(&mut self, device_kind: DeviceKind) -> u64 {
        match device_kind {
            DeviceKind::Keyboard => {
                self.seq_keyboard = self.seq_keyboard.wrapping_add(1).max(1);
                self.seq_keyboard
            }
            DeviceKind::Mouse => {
                self.seq_mouse = self.seq_mouse.wrapping_add(1).max(1);
                self.seq_mouse
            }
            DeviceKind::Gamepad(slot) => {
                self.seq_gamepad[slot] = self.seq_gamepad[slot].wrapping_add(1).max(1);
                self.seq_gamepad[slot]
            }
        }
    }

    fn baseline(&self, device_kind: DeviceKind) -> Option<&ReadingSnapshot> {
        match device_kind {
            DeviceKind::Keyboard => self.baseline_keyboard.as_ref(),
            DeviceKind::Mouse => self.baseline_mouse.as_ref(),
            DeviceKind::Gamepad(slot) => self.baseline_gamepad.get(slot).and_then(|b| b.as_ref()),
        }
    }

    fn set_baseline(
        &mut self,
        device_kind: DeviceKind,
        input_kind: u32,
        payload: ReadingPayload,
        timestamp_us: u64,
    ) {
        let seq = self.next_seq(device_kind);
        let snap = ReadingSnapshot {
            history_id: 0,
            sequence_number: seq,
            timestamp_us,
            input_kind,
            device_kind,
            payload,
        };
        match device_kind {
            DeviceKind::Keyboard => self.baseline_keyboard = Some(snap),
            DeviceKind::Mouse => self.baseline_mouse = Some(snap),
            DeviceKind::Gamepad(slot) => {
                if let Some(entry) = self.baseline_gamepad.get_mut(slot) {
                    *entry = Some(snap);
                }
            }
        }
    }

    fn clear_baseline(&mut self, device_kind: DeviceKind) {
        match device_kind {
            DeviceKind::Keyboard => self.baseline_keyboard = None,
            DeviceKind::Mouse => self.baseline_mouse = None,
            DeviceKind::Gamepad(slot) => {
                if let Some(entry) = self.baseline_gamepad.get_mut(slot) {
                    *entry = None;
                }
            }
        }
    }

    fn record_or_reuse(
        &mut self,
        device_kind: DeviceKind,
        input_kind: u32,
        payload: ReadingPayload,
    ) -> ReadingSnapshot {
        self.record_or_reuse_at(device_kind, input_kind, payload, current_timestamp_us())
    }

    fn record_or_reuse_at(
        &mut self,
        device_kind: DeviceKind,
        input_kind: u32,
        payload: ReadingPayload,
        timestamp_us: u64,
    ) -> ReadingSnapshot {
        if let Some(latest) = self
            .history
            .iter()
            .rev()
            .find(|e| e.device_kind == device_kind && e.input_kind == input_kind)
        {
            if latest.payload == payload {
                return latest.clone();
            }
        }
        self.push_snapshot_at(device_kind, input_kind, payload, timestamp_us)
    }

    fn push_snapshot(
        &mut self,
        device_kind: DeviceKind,
        input_kind: u32,
        payload: ReadingPayload,
    ) -> ReadingSnapshot {
        self.push_snapshot_at(device_kind, input_kind, payload, current_timestamp_us())
    }

    fn push_snapshot_at(
        &mut self,
        device_kind: DeviceKind,
        input_kind: u32,
        payload: ReadingPayload,
        timestamp_us: u64,
    ) -> ReadingSnapshot {
        self.next_history_id = self.next_history_id.wrapping_add(1).max(1);
        let seq = self.next_seq(device_kind);
        let snap = ReadingSnapshot {
            history_id: self.next_history_id,
            sequence_number: seq,
            timestamp_us,
            input_kind,
            device_kind,
            payload,
        };
        if self.history.len() >= READING_HISTORY_CAP {
            if let Some(evicted) = self.history.pop_front() {
                self.clear_baseline(evicted.device_kind);
            }
        }
        self.history.push_back(snap.clone());
        snap
    }

    fn alloc_callback(&mut self, kind: CallbackKind) -> u64 {
        let token = self.next_callback_token;
        self.next_callback_token = self.next_callback_token.wrapping_add(1).max(1);
        self.callbacks.insert(token, CallbackEntry { kind, active: true });
        token
    }
}

fn runtime_state() -> &'static Mutex<RuntimeState> {
    static STATE: OnceLock<Mutex<RuntimeState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(RuntimeState::new()))
}

fn kind_matches_filter(snapshot_kind: u32, filter: u32) -> bool {
    if filter == GAME_INPUT_KIND_UNKNOWN {
        return false;
    }
    if snapshot_kind & filter != 0 {
        return true;
    }
    snapshot_kind == GAME_INPUT_KIND_GAMEPAD && (filter & GAME_INPUT_KIND_CONTROLLER) != 0
}

fn device_status_for(kind: DeviceKind) -> u32 {
    match kind {
        DeviceKind::Keyboard | DeviceKind::Mouse => {
            GAME_INPUT_DEVICE_CONNECTED | GAME_INPUT_DEVICE_INPUT_ENABLED
        }
        DeviceKind::Gamepad(slot) => {
            ensure_gamepad_started();
            if xinput1_4::slot_state(slot).is_some() {
                GAME_INPUT_DEVICE_CONNECTED | GAME_INPUT_DEVICE_INPUT_ENABLED
            } else {
                GAME_INPUT_DEVICE_NO_STATUS
            }
        }
    }
}

fn poll_matching_live(
    input_kind: u32,
    device_filter: Option<DeviceKind>,
) -> Option<(DeviceKind, u32, ReadingPayload)> {
    if input_kind == GAME_INPUT_KIND_UNKNOWN {
        return None;
    }
    if let Some(dk) = device_filter {
        return match dk {
            DeviceKind::Keyboard => {
                if input_kind & GAME_INPUT_KIND_KEYBOARD != 0 {
                    Some((dk, GAME_INPUT_KIND_KEYBOARD, ReadingPayload::Keyboard(snapshot_keyboard())))
                } else {
                    None
                }
            }
            DeviceKind::Mouse => {
                if input_kind & GAME_INPUT_KIND_MOUSE != 0 {
                    Some((dk, GAME_INPUT_KIND_MOUSE, ReadingPayload::Mouse(snapshot_mouse())))
                } else {
                    None
                }
            }
            DeviceKind::Gamepad(slot) => {
                if input_kind & (GAME_INPUT_KIND_GAMEPAD | GAME_INPUT_KIND_CONTROLLER) == 0 {
                    return None;
                }
                ensure_gamepad_started();
                let (pad, _) = xinput1_4::slot_state(slot)?;
                Some((
                    dk,
                    GAME_INPUT_KIND_GAMEPAD,
                    ReadingPayload::Gamepad(xinput_to_gamepad_state(&pad)),
                ))
            }
        };
    }

    if input_kind & (GAME_INPUT_KIND_GAMEPAD | GAME_INPUT_KIND_CONTROLLER) != 0 {
        ensure_gamepad_started();
        for slot in 0..xinput1_4::XUSER_MAX_COUNT {
            if let Some((pad, _)) = xinput1_4::slot_state(slot) {
                return Some((
                    DeviceKind::Gamepad(slot),
                    GAME_INPUT_KIND_GAMEPAD,
                    ReadingPayload::Gamepad(xinput_to_gamepad_state(&pad)),
                ));
            }
        }
    }
    if input_kind & GAME_INPUT_KIND_KEYBOARD != 0 {
        return Some((
            DeviceKind::Keyboard,
            GAME_INPUT_KIND_KEYBOARD,
            ReadingPayload::Keyboard(snapshot_keyboard()),
        ));
    }
    if input_kind & GAME_INPUT_KIND_MOUSE != 0 {
        return Some((
            DeviceKind::Mouse,
            GAME_INPUT_KIND_MOUSE,
            ReadingPayload::Mouse(snapshot_mouse()),
        ));
    }
    None
}

// ---------------------------------------------------------------------------
// IGameInputDevice (22 vtable slots)
// ---------------------------------------------------------------------------

#[repr(C)]
struct DeviceObj {
    #[allow(dead_code)]
    vtbl: *const [crate::Ptr; 22],
    refs: AtomicU32,
    kind: DeviceKind,
    info: Box<GameInputDeviceInfo>,
    last_rumble: Mutex<GameInputRumbleParams>,
    ffb_gain: Mutex<f32>,
    input_sync: AtomicBool,
}

fn build_device_info(kind: DeviceKind) -> Box<GameInputDeviceInfo> {
    let base_ver = GameInputVersion { major: 1, minor: 0, build: 0, revision: 0 };
    let (
        vendor_id,
        product_id,
        usage,
        device_id,
        device_family,
        supported_input,
        supported_rumble_motors,
        axis_count,
        button_count,
        switch_count,
        keyboard_info,
        mouse_info,
        gamepad_info,
        display_name,
    ) = match kind {
        DeviceKind::Keyboard => (
            0x05ACu16,
            0x0002u16,
            GameInputUsage { page: 0x01, id: 0x06 },
            make_device_id(1, 0),
            GAME_INPUT_FAMILY_HID,
            GAME_INPUT_KIND_KEYBOARD,
            GAME_INPUT_RUMBLE_NONE,
            0u32,
            0u32,
            0u32,
            &KEYBOARD_INFO as *const GameInputKeyboardInfo,
            std::ptr::null(),
            std::ptr::null(),
            &NAME_KEYBOARD as *const GameInputString,
        ),
        DeviceKind::Mouse => (
            0x05ACu16,
            0x0001u16,
            GameInputUsage { page: 0x01, id: 0x02 },
            make_device_id(2, 0),
            GAME_INPUT_FAMILY_HID,
            GAME_INPUT_KIND_MOUSE,
            GAME_INPUT_RUMBLE_NONE,
            0u32,
            0u32,
            0u32,
            std::ptr::null(),
            &MOUSE_INFO as *const GameInputMouseInfo,
            std::ptr::null(),
            &NAME_MOUSE as *const GameInputString,
        ),
        DeviceKind::Gamepad(slot) => (
            0x045Eu16, // Microsoft
            0x02EAu16, // Xbox One S Controller
            GameInputUsage { page: 0x01, id: 0x05 },
            make_device_id(3, slot as u8),
            GAME_INPUT_FAMILY_XBOX_ONE,
            GAME_INPUT_KIND_GAMEPAD | GAME_INPUT_KIND_CONTROLLER,
            GAME_INPUT_RUMBLE_LOW_FREQUENCY
                | GAME_INPUT_RUMBLE_HIGH_FREQUENCY
                | GAME_INPUT_RUMBLE_LEFT_TRIGGER
                | GAME_INPUT_RUMBLE_RIGHT_TRIGGER,
            6u32,
            10u32,
            1u32,
            std::ptr::null(),
            std::ptr::null(),
            &GAMEPAD_INFO as *const GameInputGamepadInfo,
            &NAME_GAMEPAD[slot] as *const GameInputString,
        ),
    };

    Box::new(GameInputDeviceInfo {
        info_size: std::mem::size_of::<GameInputDeviceInfo>() as u32,
        vendor_id,
        product_id,
        revision_number: 1,
        interface_number: 0,
        collection_number: 0,
        usage,
        hardware_version: base_ver,
        firmware_version: base_ver,
        device_id,
        device_root_id: device_id,
        device_family,
        capabilities: 0,
        supported_input,
        supported_rumble_motors,
        input_report_count: 0,
        output_report_count: 0,
        feature_report_count: 0,
        controller_axis_count: axis_count,
        controller_button_count: button_count,
        controller_switch_count: switch_count,
        touch_point_count: 0,
        touch_sensor_count: 0,
        force_feedback_motor_count: 0,
        haptic_feedback_motor_count: 0,
        device_string_count: 1,
        device_descriptor_size: 0,
        input_report_info: std::ptr::null(),
        output_report_info: std::ptr::null(),
        feature_report_info: std::ptr::null(),
        controller_axis_info: std::ptr::null(),
        controller_button_info: std::ptr::null(),
        controller_switch_info: std::ptr::null(),
        keyboard_info,
        mouse_info,
        touch_sensor_info: std::ptr::null(),
        motion_info: std::ptr::null(),
        arcade_stick_info: std::ptr::null(),
        flight_stick_info: std::ptr::null(),
        gamepad_info,
        racing_wheel_info: std::ptr::null(),
        force_feedback_motor_info: std::ptr::null(),
        haptic_feedback_motor_info: std::ptr::null(),
        display_name,
        device_strings: display_name,
        device_descriptor_data: std::ptr::null(),
        supported_system_buttons: 0,
        _pad_end: 0,
    })
}

fn alloc_device(kind: DeviceKind) -> *mut DeviceObj {
    Box::into_raw(Box::new(DeviceObj {
        vtbl: &DEVICE_VTBL,
        refs: AtomicU32::new(1),
        kind,
        info: build_device_info(kind),
        last_rumble: Mutex::new(GameInputRumbleParams::default()),
        ffb_gain: Mutex::new(1.0),
        input_sync: AtomicBool::new(false),
    }))
}

fn device_index(kind: DeviceKind) -> usize {
    match kind {
        DeviceKind::Keyboard => 0,
        DeviceKind::Mouse => 1,
        DeviceKind::Gamepad(slot) => 2 + slot,
    }
}

fn persistent_devices() -> &'static [usize; 6] {
    static DEVICES: OnceLock<[usize; 6]> = OnceLock::new();
    DEVICES.get_or_init(|| {
        [
            alloc_device(DeviceKind::Keyboard) as usize,
            alloc_device(DeviceKind::Mouse) as usize,
            alloc_device(DeviceKind::Gamepad(0)) as usize,
            alloc_device(DeviceKind::Gamepad(1)) as usize,
            alloc_device(DeviceKind::Gamepad(2)) as usize,
            alloc_device(DeviceKind::Gamepad(3)) as usize,
        ]
    })
}

fn device_ptr(kind: DeviceKind) -> *mut DeviceObj {
    persistent_devices()[device_index(kind)] as *mut DeviceObj
}

fn lookup_device_kind(ptr: *mut DeviceObj) -> Option<DeviceKind> {
    if ptr.is_null() {
        return None;
    }
    let addr = ptr as usize;
    let devs = persistent_devices();
    if addr == devs[0] {
        Some(DeviceKind::Keyboard)
    } else if addr == devs[1] {
        Some(DeviceKind::Mouse)
    } else {
        (0..xinput1_4::XUSER_MAX_COUNT)
            .find(|&slot| addr == devs[2 + slot])
            .map(DeviceKind::Gamepad)
    }
}

unsafe extern "win64" fn dev_query_interface(
    this: *mut DeviceObj,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> i32 {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    if this.is_null() || riid.is_null() {
        return E_POINTER;
    }
    let g = std::ptr::read_unaligned(riid);
    if g == IID_IUNKNOWN || g == IID_IGAMEINPUT_DEVICE {
        dev_add_ref(this);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn dev_add_ref(this: *mut DeviceObj) -> u32 {
    if this.is_null() {
        return 0;
    }
    (*this).refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn dev_release(this: *mut DeviceObj) -> u32 {
    if this.is_null() {
        return 0;
    }
    // Persistent devices keep a runtime base reference of 1 so they are never freed prematurely.
    loop {
        let cur = (*this).refs.load(Ordering::Acquire);
        if cur <= 1 {
            return 1;
        }
        if (*this)
            .refs
            .compare_exchange(cur, cur - 1, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            return cur - 1;
        }
    }
}

unsafe extern "win64" fn dev_get_device_info(this: *mut DeviceObj) -> *const GameInputDeviceInfo {
    if this.is_null() {
        return std::ptr::null();
    }
    &*(*this).info as *const GameInputDeviceInfo
}

unsafe extern "win64" fn dev_get_device_status(this: *mut DeviceObj) -> u32 {
    if this.is_null() {
        return GAME_INPUT_DEVICE_NO_STATUS;
    }
    device_status_for((*this).kind)
}

unsafe extern "win64" fn dev_get_battery_state(
    this: *mut DeviceObj,
    state_out: *mut GameInputBatteryState,
) {
    if state_out.is_null() {
        return;
    }
    let mut bs = GameInputBatteryState {
        charge_rate: 0.0,
        max_charge_rate: 0.0,
        remaining_capacity: 0.0,
        full_charge_capacity: 0.0,
        status: GAME_INPUT_BATTERY_NOT_PRESENT,
    };
    if !this.is_null() {
        if let DeviceKind::Gamepad(slot) = (*this).kind {
            let mut bi = xinput1_4::BatteryInformation::default();
            if xinput1_4::XInputGetBatteryInformation(slot as u32, 0, &mut bi) == 0 && bi.ty != 0 && bi.ty != 1 {
                let frac = match bi.level {
                    0 => 0.05,
                    1 => 0.30,
                    2 => 0.65,
                    _ => 1.0,
                };
                bs.status = GAME_INPUT_BATTERY_DISCHARGING;
                bs.remaining_capacity = frac;
                bs.full_charge_capacity = 1.0;
            }
        }
    }
    std::ptr::write_unaligned(state_out, bs);
}

unsafe extern "win64" fn dev_create_force_feedback_effect(
    _this: *mut DeviceObj,
    _motor_index: u32,
    _params: *const c_void,
    effect_out: *mut *mut c_void,
) -> i32 {
    if !effect_out.is_null() {
        std::ptr::write_unaligned(effect_out, std::ptr::null_mut());
    }
    E_NOTIMPL
}

unsafe extern "win64" fn dev_is_force_feedback_motor_powered_on(
    _this: *mut DeviceObj,
    _motor_index: u32,
) -> u8 {
    0
}

unsafe extern "win64" fn dev_set_force_feedback_motor_gain(
    this: *mut DeviceObj,
    motor_index: u32,
    master_gain: f32,
) {
    if this.is_null() {
        return;
    }
    let gain = clamp_unit(master_gain);
    let mut guard = (*this).ffb_gain.lock().unwrap();
    let prev = *guard;
    *guard = gain;
    if (prev - gain).abs() > f32::EPSILON {
        log::trace!("gameinput: device {:?} motor {motor_index} ffb gain {prev:.2} -> {gain:.2}", (*this).kind);
    }
}
crate::win64_bridge!(fn dev_set_force_feedback_motor_gain(*mut DeviceObj, u32, f32) -> ());

unsafe extern "win64" fn dev_set_haptic_motor_state(
    _this: *mut DeviceObj,
    _motor_index: u32,
    _params: *const c_void,
) -> i32 {
    E_NOTIMPL
}

unsafe extern "win64" fn dev_set_rumble_state(
    this: *mut DeviceObj,
    params: *const GameInputRumbleParams,
) {
    if this.is_null() {
        return;
    }
    let raw = if params.is_null() {
        GameInputRumbleParams::default()
    } else {
        std::ptr::read_unaligned(params)
    };
    let clamped = GameInputRumbleParams {
        low_frequency: clamp_unit(raw.low_frequency),
        high_frequency: clamp_unit(raw.high_frequency),
        left_trigger: clamp_unit(raw.left_trigger),
        right_trigger: clamp_unit(raw.right_trigger),
    };
    {
        let mut guard = (*this).last_rumble.lock().unwrap();
        let prev = *guard;
        *guard = clamped;
        if prev != clamped {
            log::trace!(
                "gameinput: SetRumbleState({:?}) low={:.2} high={:.2} lt={:.2} rt={:.2}",
                (*this).kind,
                clamped.low_frequency,
                clamped.high_frequency,
                clamped.left_trigger,
                clamped.right_trigger
            );
        }
    }
    if let DeviceKind::Gamepad(slot) = (*this).kind {
        let (left, right) = rumble_params_to_xinput(clamped);
        let mut vib = xinput1_4::Vibration { left, right };
        let _ = xinput1_4::XInputSetState(slot as u32, &mut vib);
    }
}

unsafe extern "win64" fn dev_set_input_synchronization_state(this: *mut DeviceObj, enabled: u8) {
    if !this.is_null() {
        let _prev = (*this).input_sync.swap(enabled != 0, Ordering::AcqRel);
    }
}

unsafe extern "win64" fn dev_send_input_synchronization_hint(_this: *mut DeviceObj) {}

unsafe extern "win64" fn dev_power_off(_this: *mut DeviceObj) {}

unsafe extern "win64" fn dev_create_raw_device_report(
    _this: *mut DeviceObj,
    _report_id: u32,
    _report_kind: u32,
    report_out: *mut *mut c_void,
) -> i32 {
    if !report_out.is_null() {
        std::ptr::write_unaligned(report_out, std::ptr::null_mut());
    }
    E_NOTIMPL
}

unsafe extern "win64" fn dev_get_raw_device_feature(
    _this: *mut DeviceObj,
    _report_id: u32,
    report_out: *mut *mut c_void,
) -> i32 {
    if !report_out.is_null() {
        std::ptr::write_unaligned(report_out, std::ptr::null_mut());
    }
    E_NOTIMPL
}

unsafe extern "win64" fn dev_set_raw_device_feature(
    _this: *mut DeviceObj,
    _report: *mut c_void,
) -> i32 {
    E_NOTIMPL
}

unsafe extern "win64" fn dev_send_raw_device_output(
    _this: *mut DeviceObj,
    _report: *mut c_void,
) -> i32 {
    E_NOTIMPL
}

unsafe extern "win64" fn dev_send_raw_device_output_with_response(
    _this: *mut DeviceObj,
    _request: *mut c_void,
    response_out: *mut *mut c_void,
) -> i32 {
    if !response_out.is_null() {
        std::ptr::write_unaligned(response_out, std::ptr::null_mut());
    }
    E_NOTIMPL
}

unsafe extern "win64" fn dev_execute_raw_device_ioctl(
    _this: *mut DeviceObj,
    _control_code: u32,
    _input_size: usize,
    _input_buf: *const c_void,
    _output_size: usize,
    _output_buf: *mut c_void,
    output_size_out: *mut usize,
) -> i32 {
    if !output_size_out.is_null() {
        std::ptr::write_unaligned(output_size_out, 0);
    }
    E_NOTIMPL
}

unsafe extern "win64" fn dev_acquire_exclusive_raw_device_access(
    _this: *mut DeviceObj,
    _timeout_us: u64,
) -> u8 {
    0
}

unsafe extern "win64" fn dev_release_exclusive_raw_device_access(_this: *mut DeviceObj) {}

static DEVICE_VTBL: [crate::Ptr; 22] = [
    crate::Ptr(dev_query_interface as *const ()),                       //  0 (+0x00) QueryInterface
    crate::Ptr(dev_add_ref as *const ()),                               //  1 (+0x08) AddRef
    crate::Ptr(dev_release as *const ()),                               //  2 (+0x10) Release
    crate::Ptr(dev_get_device_info as *const ()),                       //  3 (+0x18) GetDeviceInfo
    crate::Ptr(dev_get_device_status as *const ()),                     //  4 (+0x20) GetDeviceStatus
    crate::Ptr(dev_get_battery_state as *const ()),                     //  5 (+0x28) GetBatteryState
    crate::Ptr(dev_create_force_feedback_effect as *const ()),          //  6 (+0x30) CreateForceFeedbackEffect
    crate::Ptr(dev_is_force_feedback_motor_powered_on as *const ()),    //  7 (+0x38) IsForceFeedbackMotorPoweredOn
    crate::Ptr(crate::bridge_entry!(dev_set_force_feedback_motor_gain)),//  8 (+0x40) SetForceFeedbackMotorGain
    crate::Ptr(dev_set_haptic_motor_state as *const ()),                //  9 (+0x48) SetHapticMotorState
    crate::Ptr(dev_set_rumble_state as *const ()),                      // 10 (+0x50) SetRumbleState
    crate::Ptr(dev_set_input_synchronization_state as *const ()),       // 11 (+0x58) SetInputSynchronizationState
    crate::Ptr(dev_send_input_synchronization_hint as *const ()),       // 12 (+0x60) SendInputSynchronizationHint
    crate::Ptr(dev_power_off as *const ()),                             // 13 (+0x68) PowerOff
    crate::Ptr(dev_create_raw_device_report as *const ()),              // 14 (+0x70) CreateRawDeviceReport
    crate::Ptr(dev_get_raw_device_feature as *const ()),                // 15 (+0x78) GetRawDeviceFeature
    crate::Ptr(dev_set_raw_device_feature as *const ()),                // 16 (+0x80) SetRawDeviceFeature
    crate::Ptr(dev_send_raw_device_output as *const ()),                // 17 (+0x88) SendRawDeviceOutput
    crate::Ptr(dev_send_raw_device_output_with_response as *const ()),  // 18 (+0x90) SendRawDeviceOutputWithResponse
    crate::Ptr(dev_execute_raw_device_ioctl as *const ()),              // 19 (+0x98) ExecuteRawDeviceIoControl
    crate::Ptr(dev_acquire_exclusive_raw_device_access as *const ()),   // 20 (+0xA0) AcquireExclusiveRawDeviceAccess
    crate::Ptr(dev_release_exclusive_raw_device_access as *const ()),   // 21 (+0xA8) ReleaseExclusiveRawDeviceAccess
];

// ---------------------------------------------------------------------------
// IGameInputReading (25 vtable slots)
// ---------------------------------------------------------------------------

#[repr(C)]
struct ReadingObj {
    #[allow(dead_code)]
    vtbl: *const [crate::Ptr; 25],
    refs: AtomicU32,
    snapshot: ReadingSnapshot,
}

impl ReadingObj {
    fn create(snapshot: ReadingSnapshot) -> *mut Self {
        Box::into_raw(Box::new(Self {
            vtbl: &READING_VTBL,
            refs: AtomicU32::new(1),
            snapshot,
        }))
    }
}

unsafe extern "win64" fn rd_query_interface(
    this: *mut ReadingObj,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> i32 {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    if this.is_null() || riid.is_null() {
        return E_POINTER;
    }
    let g = std::ptr::read_unaligned(riid);
    if g == IID_IUNKNOWN || g == IID_IGAMEINPUT_READING {
        rd_add_ref(this);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn rd_add_ref(this: *mut ReadingObj) -> u32 {
    if this.is_null() {
        return 0;
    }
    (*this).refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn rd_release(this: *mut ReadingObj) -> u32 {
    if this.is_null() {
        return 0;
    }
    let n = (*this).refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if n == 0 {
        drop(Box::from_raw(this));
    }
    n
}

unsafe extern "win64" fn rd_get_input_kind(this: *mut ReadingObj) -> u32 {
    if this.is_null() {
        return GAME_INPUT_KIND_UNKNOWN;
    }
    (*this).snapshot.input_kind
}

unsafe extern "win64" fn rd_get_sequence_number(this: *mut ReadingObj, input_kind: u32) -> u64 {
    if this.is_null() {
        return 0;
    }
    let snap = &(*this).snapshot;
    if input_kind == GAME_INPUT_KIND_UNKNOWN || kind_matches_filter(snap.input_kind, input_kind) {
        snap.sequence_number
    } else {
        0
    }
}

unsafe extern "win64" fn rd_get_timestamp(this: *mut ReadingObj) -> u64 {
    if this.is_null() {
        return 0;
    }
    (*this).snapshot.timestamp_us
}

unsafe extern "win64" fn rd_get_device(this: *mut ReadingObj, device_out: *mut *mut DeviceObj) {
    if device_out.is_null() {
        return;
    }
    if this.is_null() {
        std::ptr::write_unaligned(device_out, std::ptr::null_mut());
        return;
    }
    let dev = device_ptr((*this).snapshot.device_kind);
    dev_add_ref(dev);
    std::ptr::write_unaligned(device_out, dev);
}

unsafe extern "win64" fn rd_get_raw_report(
    _this: *mut ReadingObj,
    report_out: *mut *mut c_void,
) -> u8 {
    if !report_out.is_null() {
        std::ptr::write_unaligned(report_out, std::ptr::null_mut());
    }
    0
}

unsafe extern "win64" fn rd_get_controller_axis_count(this: *mut ReadingObj) -> u32 {
    if this.is_null() {
        return 0;
    }
    match (*this).snapshot.payload {
        ReadingPayload::Gamepad(_) => 6,
        _ => 0,
    }
}

unsafe extern "win64" fn rd_get_controller_axis_state(
    this: *mut ReadingObj,
    state_array_count: u32,
    state_array: *mut f32,
) -> u32 {
    if this.is_null() || state_array.is_null() || state_array_count == 0 {
        return 0;
    }
    let ReadingPayload::Gamepad(gp) = (*this).snapshot.payload else {
        return 0;
    };
    let axes = [
        (gp.left_thumbstick_x + 1.0) * 0.5,
        (gp.left_thumbstick_y + 1.0) * 0.5,
        (gp.right_thumbstick_x + 1.0) * 0.5,
        (gp.right_thumbstick_y + 1.0) * 0.5,
        gp.left_trigger,
        gp.right_trigger,
    ];
    let n = axes.len().min(state_array_count as usize);
    for (i, &v) in axes[..n].iter().enumerate() {
        std::ptr::write_unaligned(state_array.add(i), v);
    }
    n as u32
}

unsafe extern "win64" fn rd_get_controller_button_count(this: *mut ReadingObj) -> u32 {
    if this.is_null() {
        return 0;
    }
    match (*this).snapshot.payload {
        ReadingPayload::Gamepad(_) => 10,
        _ => 0,
    }
}

unsafe extern "win64" fn rd_get_controller_button_state(
    this: *mut ReadingObj,
    state_array_count: u32,
    state_array: *mut u8,
) -> u32 {
    if this.is_null() || state_array.is_null() || state_array_count == 0 {
        return 0;
    }
    let ReadingPayload::Gamepad(gp) = (*this).snapshot.payload else {
        return 0;
    };
    let bits = [
        GAME_INPUT_GAMEPAD_MENU,
        GAME_INPUT_GAMEPAD_VIEW,
        GAME_INPUT_GAMEPAD_A,
        GAME_INPUT_GAMEPAD_B,
        GAME_INPUT_GAMEPAD_X,
        GAME_INPUT_GAMEPAD_Y,
        GAME_INPUT_GAMEPAD_LEFT_SHOULDER,
        GAME_INPUT_GAMEPAD_RIGHT_SHOULDER,
        GAME_INPUT_GAMEPAD_LEFT_THUMBSTICK,
        GAME_INPUT_GAMEPAD_RIGHT_THUMBSTICK,
    ];
    let n = bits.len().min(state_array_count as usize);
    for (i, &bit) in bits[..n].iter().enumerate() {
        std::ptr::write_unaligned(state_array.add(i), u8::from((gp.buttons & bit) != 0));
    }
    n as u32
}

unsafe extern "win64" fn rd_get_controller_switch_count(this: *mut ReadingObj) -> u32 {
    if this.is_null() {
        return 0;
    }
    match (*this).snapshot.payload {
        ReadingPayload::Gamepad(_) => 1,
        _ => 0,
    }
}

unsafe extern "win64" fn rd_get_controller_switch_state(
    this: *mut ReadingObj,
    state_array_count: u32,
    state_array: *mut u32,
) -> u32 {
    if this.is_null() || state_array.is_null() || state_array_count == 0 {
        return 0;
    }
    let ReadingPayload::Gamepad(gp) = (*this).snapshot.payload else {
        return 0;
    };
    std::ptr::write_unaligned(state_array, dpad_to_switch(gp.buttons));
    1
}

unsafe extern "win64" fn rd_get_key_count(this: *mut ReadingObj) -> u32 {
    if this.is_null() {
        return 0;
    }
    match &(*this).snapshot.payload {
        ReadingPayload::Keyboard(keys) => keys.len() as u32,
        _ => 0,
    }
}

unsafe extern "win64" fn rd_get_key_state(
    this: *mut ReadingObj,
    state_array_count: u32,
    state_array: *mut GameInputKeyState,
) -> u32 {
    if this.is_null() || state_array.is_null() || state_array_count == 0 {
        return 0;
    }
    let ReadingPayload::Keyboard(keys) = &(*this).snapshot.payload else {
        return 0;
    };
    let n = keys.len().min(state_array_count as usize);
    for (i, &k) in keys[..n].iter().enumerate() {
        std::ptr::write_unaligned(state_array.add(i), k);
    }
    n as u32
}

unsafe extern "win64" fn rd_get_mouse_state(
    this: *mut ReadingObj,
    state_out: *mut GameInputMouseState,
) -> u8 {
    if this.is_null() || state_out.is_null() {
        return 0;
    }
    let ReadingPayload::Mouse(st) = (*this).snapshot.payload else {
        return 0;
    };
    std::ptr::write_unaligned(state_out, st);
    1
}

unsafe extern "win64" fn rd_get_touch_count(_this: *mut ReadingObj) -> u32 {
    0
}

unsafe extern "win64" fn rd_get_touch_state(
    _this: *mut ReadingObj,
    _count: u32,
    _state: *mut c_void,
) -> u32 {
    0
}

unsafe extern "win64" fn rd_get_unsupported_state(
    _this: *mut ReadingObj,
    _state: *mut c_void,
) -> u8 {
    0
}

unsafe extern "win64" fn rd_get_gamepad_state(
    this: *mut ReadingObj,
    state_out: *mut GameInputGamepadState,
) -> u8 {
    if this.is_null() || state_out.is_null() {
        return 0;
    }
    let ReadingPayload::Gamepad(st) = (*this).snapshot.payload else {
        return 0;
    };
    std::ptr::write_unaligned(state_out, st);
    1
}

static READING_VTBL: [crate::Ptr; 25] = [
    crate::Ptr(rd_query_interface as *const ()),             //  0 (+0x00) QueryInterface
    crate::Ptr(rd_add_ref as *const ()),                     //  1 (+0x08) AddRef
    crate::Ptr(rd_release as *const ()),                     //  2 (+0x10) Release
    crate::Ptr(rd_get_input_kind as *const ()),              //  3 (+0x18) GetInputKind
    crate::Ptr(rd_get_sequence_number as *const ()),         //  4 (+0x20) GetSequenceNumber
    crate::Ptr(rd_get_timestamp as *const ()),               //  5 (+0x28) GetTimestamp
    crate::Ptr(rd_get_device as *const ()),                  //  6 (+0x30) GetDevice
    crate::Ptr(rd_get_raw_report as *const ()),              //  7 (+0x38) GetRawReport
    crate::Ptr(rd_get_controller_axis_count as *const ()),   //  8 (+0x40) GetControllerAxisCount
    crate::Ptr(rd_get_controller_axis_state as *const ()),   //  9 (+0x48) GetControllerAxisState
    crate::Ptr(rd_get_controller_button_count as *const ()), // 10 (+0x50) GetControllerButtonCount
    crate::Ptr(rd_get_controller_button_state as *const ()), // 11 (+0x58) GetControllerButtonState
    crate::Ptr(rd_get_controller_switch_count as *const ()), // 12 (+0x60) GetControllerSwitchCount
    crate::Ptr(rd_get_controller_switch_state as *const ()), // 13 (+0x68) GetControllerSwitchState
    crate::Ptr(rd_get_key_count as *const ()),               // 14 (+0x70) GetKeyCount
    crate::Ptr(rd_get_key_state as *const ()),               // 15 (+0x78) GetKeyState
    crate::Ptr(rd_get_mouse_state as *const ()),             // 16 (+0x80) GetMouseState
    crate::Ptr(rd_get_touch_count as *const ()),             // 17 (+0x88) GetTouchCount
    crate::Ptr(rd_get_touch_state as *const ()),             // 18 (+0x90) GetTouchState
    crate::Ptr(rd_get_unsupported_state as *const ()),       // 19 (+0x98) GetMotionState
    crate::Ptr(rd_get_unsupported_state as *const ()),       // 20 (+0xA0) GetArcadeStickState
    crate::Ptr(rd_get_unsupported_state as *const ()),       // 21 (+0xA8) GetFlightStickState
    crate::Ptr(rd_get_gamepad_state as *const ()),           // 22 (+0xB0) GetGamepadState
    crate::Ptr(rd_get_unsupported_state as *const ()),       // 23 (+0xB8) GetRacingWheelState
    crate::Ptr(rd_get_unsupported_state as *const ()),       // 24 (+0xC0) GetUiNavigationState
];

// ---------------------------------------------------------------------------
// IGameInputDispatcher (5 vtable slots)
// ---------------------------------------------------------------------------

#[repr(C)]
struct DispatcherObj {
    #[allow(dead_code)]
    vtbl: *const [crate::Ptr; 5],
    refs: AtomicU32,
}

unsafe extern "win64" fn disp_query_interface(
    this: *mut DispatcherObj,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> i32 {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    if this.is_null() || riid.is_null() {
        return E_POINTER;
    }
    let g = std::ptr::read_unaligned(riid);
    if g == IID_IUNKNOWN || g == IID_IGAMEINPUT_DISPATCHER {
        disp_add_ref(this);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn disp_add_ref(this: *mut DispatcherObj) -> u32 {
    if this.is_null() {
        return 0;
    }
    (*this).refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn disp_release(this: *mut DispatcherObj) -> u32 {
    if this.is_null() {
        return 0;
    }
    let n = (*this).refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if n == 0 {
        drop(Box::from_raw(this));
    }
    n
}

unsafe extern "win64" fn disp_dispatch(_this: *mut DispatcherObj, _quota_us: u64) -> u8 {
    0
}

unsafe extern "win64" fn disp_open_wait_handle(
    _this: *mut DispatcherObj,
    wait_handle_out: *mut usize,
) -> i32 {
    if !wait_handle_out.is_null() {
        std::ptr::write_unaligned(wait_handle_out, 0);
    }
    E_NOTIMPL
}

static DISPATCHER_VTBL: [crate::Ptr; 5] = [
    crate::Ptr(disp_query_interface as *const ()),  // 0 (+0x00) QueryInterface
    crate::Ptr(disp_add_ref as *const ()),          // 1 (+0x08) AddRef
    crate::Ptr(disp_release as *const ()),          // 2 (+0x10) Release
    crate::Ptr(disp_dispatch as *const ()),         // 3 (+0x18) Dispatch
    crate::Ptr(disp_open_wait_handle as *const ()), // 4 (+0x20) OpenWaitHandle
];

// ---------------------------------------------------------------------------
// IGameInput (22 vtable slots)
// ---------------------------------------------------------------------------

#[repr(C)]
struct GameInputObj {
    #[allow(dead_code)]
    vtbl: *const [crate::Ptr; 22],
    refs: AtomicU32,
}

unsafe extern "win64" fn gi_query_interface(
    this: *mut GameInputObj,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> i32 {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    if this.is_null() || riid.is_null() {
        return E_POINTER;
    }
    let g = std::ptr::read_unaligned(riid);
    if g == IID_IUNKNOWN || g == IID_IGAMEINPUT {
        gi_add_ref(this);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn gi_add_ref(this: *mut GameInputObj) -> u32 {
    if this.is_null() {
        return 0;
    }
    (*this).refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn gi_release(this: *mut GameInputObj) -> u32 {
    if this.is_null() {
        return 0;
    }
    let n = (*this).refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if n == 0 {
        drop(Box::from_raw(this));
    }
    n
}

unsafe extern "win64" fn gi_get_current_timestamp(_this: *mut GameInputObj) -> u64 {
    current_timestamp_us()
}

unsafe extern "win64" fn gi_get_current_reading(
    _this: *mut GameInputObj,
    input_kind: u32,
    device: *mut DeviceObj,
    reading_out: *mut *mut ReadingObj,
) -> i32 {
    if reading_out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(reading_out, std::ptr::null_mut());

    let device_filter = if device.is_null() {
        None
    } else {
        match lookup_device_kind(device) {
            Some(dk) => Some(dk),
            None => return GAMEINPUT_E_READING_NOT_FOUND,
        }
    };

    let Some((dev_kind, actual_kind, payload)) = poll_matching_live(input_kind, device_filter) else {
        return GAMEINPUT_E_READING_NOT_FOUND;
    };
    let snap = runtime_state()
        .lock()
        .unwrap()
        .record_or_reuse(dev_kind, actual_kind, payload);
    std::ptr::write_unaligned(reading_out, ReadingObj::create(snap));
    S_OK
}

unsafe extern "win64" fn gi_get_next_reading(
    _this: *mut GameInputObj,
    ref_reading: *mut ReadingObj,
    input_kind: u32,
    device: *mut DeviceObj,
    reading_out: *mut *mut ReadingObj,
) -> i32 {
    if reading_out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(reading_out, std::ptr::null_mut());
    if ref_reading.is_null() {
        return E_INVALIDARG;
    }
    let device_filter = if device.is_null() {
        None
    } else {
        match lookup_device_kind(device) {
            Some(dk) => Some(dk),
            None => return GAMEINPUT_E_READING_NOT_FOUND,
        }
    };
    let ref_id = (*ref_reading).snapshot.history_id;

    {
        let st = runtime_state().lock().unwrap();
        if let Some(found) = st.history.iter().find(|e| {
            e.history_id > ref_id
                && kind_matches_filter(e.input_kind, input_kind)
                && device_filter.map_or(true, |dk| e.device_kind == dk)
        }) {
            std::ptr::write_unaligned(reading_out, ReadingObj::create(found.clone()));
            return S_OK;
        }
        if ref_id != 0 {
            if let Some(oldest) = st.history.front() {
                if ref_id < oldest.history_id {
                    return GAMEINPUT_E_REFERENCE_READING_TOO_OLD;
                }
            }
        }
    }

    // At the newest boundary: check once if live input changed since the latest recorded snapshot.
    if let Some((dev_kind, actual_kind, live_payload)) = poll_matching_live(input_kind, device_filter) {
        let mut st = runtime_state().lock().unwrap();
        let latest_payload = st
            .history
            .iter()
            .rev()
            .find(|e| e.device_kind == dev_kind && e.input_kind == actual_kind)
            .map(|e| &e.payload)
            .or_else(|| st.baseline(dev_kind).map(|b| &b.payload));
        let unchanged = latest_payload.is_some_and(|p| *p == live_payload);
        if !unchanged {
            let snap = st.push_snapshot(dev_kind, actual_kind, live_payload);
            if snap.history_id > ref_id {
                std::ptr::write_unaligned(reading_out, ReadingObj::create(snap));
                return S_OK;
            }
        }
    }
    GAMEINPUT_E_READING_NOT_FOUND
}

unsafe extern "win64" fn gi_get_previous_reading(
    _this: *mut GameInputObj,
    ref_reading: *mut ReadingObj,
    input_kind: u32,
    device: *mut DeviceObj,
    reading_out: *mut *mut ReadingObj,
) -> i32 {
    if reading_out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(reading_out, std::ptr::null_mut());
    if ref_reading.is_null() {
        return E_INVALIDARG;
    }
    let device_filter = if device.is_null() {
        None
    } else {
        match lookup_device_kind(device) {
            Some(dk) => Some(dk),
            None => return GAMEINPUT_E_READING_NOT_FOUND,
        }
    };
    let ref_id = (*ref_reading).snapshot.history_id;

    let st = runtime_state().lock().unwrap();
    if let Some(found) = st.history.iter().rev().find(|e| {
        e.history_id < ref_id
            && kind_matches_filter(e.input_kind, input_kind)
            && device_filter.map_or(true, |dk| e.device_kind == dk)
    }) {
        std::ptr::write_unaligned(reading_out, ReadingObj::create(found.clone()));
        return S_OK;
    }
    if ref_id != 0 {
        if let Some(oldest) = st.history.front() {
            if ref_id < oldest.history_id {
                return GAMEINPUT_E_REFERENCE_READING_TOO_OLD;
            }
        }
    }
    GAMEINPUT_E_READING_NOT_FOUND
}

unsafe extern "win64" fn gi_get_temporal_reading(
    _this: *mut GameInputObj,
    timestamp: u64,
    device: *mut DeviceObj,
    reading_out: *mut *mut ReadingObj,
) -> i32 {
    if reading_out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(reading_out, std::ptr::null_mut());
    // Per Microsoft Learn IGameInput::GetTemporalReading: device must be non-null.
    if device.is_null() {
        return E_INVALIDARG;
    }
    let Some(dev_kind) = lookup_device_kind(device) else {
        return GAMEINPUT_E_READING_NOT_FOUND;
    };

    // Search the device's recorded history for the reading active at `timestamp`
    // (the latest reading with `timestamp_us <= timestamp`). Input transitions are
    // stamped when the backend receives them (`on_gamepad_state_changed`,
    // `on_keyboard_changed`, `on_mouse_changed`), and each connected device's initial
    // baseline state is captured at connection/initialization time, so historical
    // queries never depend on an earlier polling call.
    let st = runtime_state().lock().unwrap();
    let mut best: Option<&ReadingSnapshot> = None;
    for entry in st.history.iter().filter(|e| e.device_kind == dev_kind) {
        if entry.timestamp_us <= timestamp {
            best = Some(entry);
        }
    }
    if let Some(found) = best {
        std::ptr::write_unaligned(reading_out, ReadingObj::create(found.clone()));
        return S_OK;
    }
    if let Some(base) = st.baseline(dev_kind) {
        if base.timestamp_us <= timestamp {
            std::ptr::write_unaligned(reading_out, ReadingObj::create(base.clone()));
            return S_OK;
        }
    }
    GAMEINPUT_E_READING_NOT_FOUND
}

unsafe extern "win64" fn gi_register_reading_callback(
    _this: *mut GameInputObj,
    _device: *mut DeviceObj,
    input_kind: u32,
    analog_threshold: f32,
    _context: usize,
    callback_func: usize,
    callback_token: *mut u64,
) -> i32 {
    if callback_func == 0 {
        return E_INVALIDARG;
    }
    let token = runtime_state().lock().unwrap().alloc_callback(CallbackKind::Reading);
    if !callback_token.is_null() {
        std::ptr::write_unaligned(callback_token, token);
    }
    log::debug!(
        "gameinput: RegisterReadingCallback(kind={input_kind:#x}, threshold={analog_threshold:.3}) -> token={token}"
    );
    S_OK
}
crate::win64_bridge!(fn gi_register_reading_callback(*mut GameInputObj, *mut DeviceObj, u32, f32, usize, usize, *mut u64) -> i32);

unsafe extern "win64" fn gi_register_device_callback(
    _this: *mut GameInputObj,
    device: *mut DeviceObj,
    input_kind: u32,
    status_filter: u32,
    enum_kind: u32,
    context: usize,
    callback_func: usize,
    callback_token: *mut u64,
) -> i32 {
    if callback_func == 0 {
        return E_INVALIDARG;
    }
    ensure_gamepad_started();
    let token = runtime_state().lock().unwrap().alloc_callback(CallbackKind::Device);
    if !callback_token.is_null() {
        std::ptr::write_unaligned(callback_token, token);
    }

    if enum_kind == GAME_INPUT_ASYNC_ENUMERATION || enum_kind == GAME_INPUT_BLOCKING_ENUMERATION {
        let mut to_notify: Vec<(*mut DeviceObj, u32)> = Vec::new();
        for &raw in persistent_devices() {
            let dev = raw as *mut DeviceObj;
            if !device.is_null() && dev != device {
                continue;
            }
            let supported = (*dev).info.supported_input;
            if input_kind != GAME_INPUT_KIND_UNKNOWN && (supported & input_kind) == 0 {
                continue;
            }
            let status = device_status_for((*dev).kind);
            if status != GAME_INPUT_DEVICE_NO_STATUS && (status & status_filter) != 0 {
                to_notify.push((dev, status));
            }
        }
        for (dev, status) in to_notify {
            let still_active = runtime_state()
                .lock()
                .unwrap()
                .callbacks
                .get(&token)
                .is_some_and(|e| e.active);
            if !still_active {
                break;
            }
            let ts = current_timestamp_us();
            let _ = crate::call_guest8(
                callback_func,
                token,
                context as u64,
                dev as u64,
                ts,
                status as u64,
                GAME_INPUT_DEVICE_NO_STATUS as u64,
                0,
                0,
            );
        }
    }
    S_OK
}

unsafe extern "win64" fn gi_register_system_button_callback(
    _this: *mut GameInputObj,
    _device: *mut DeviceObj,
    _button_filter: u32,
    _context: usize,
    callback_func: usize,
    callback_token: *mut u64,
) -> i32 {
    if callback_func == 0 {
        return E_INVALIDARG;
    }
    let token = runtime_state().lock().unwrap().alloc_callback(CallbackKind::SystemButton);
    if !callback_token.is_null() {
        std::ptr::write_unaligned(callback_token, token);
    }
    S_OK
}

unsafe extern "win64" fn gi_register_keyboard_layout_callback(
    _this: *mut GameInputObj,
    _device: *mut DeviceObj,
    _context: usize,
    callback_func: usize,
    callback_token: *mut u64,
) -> i32 {
    if callback_func == 0 {
        return E_INVALIDARG;
    }
    let token = runtime_state().lock().unwrap().alloc_callback(CallbackKind::KeyboardLayout);
    if !callback_token.is_null() {
        std::ptr::write_unaligned(callback_token, token);
    }
    S_OK
}

unsafe extern "win64" fn gi_stop_callback(_this: *mut GameInputObj, token: u64) {
    let mut st = runtime_state().lock().unwrap();
    if let Some(entry) = st.callbacks.get_mut(&token) {
        entry.active = false;
        log::debug!("gameinput: StopCallback(token={token}, kind={:?})", entry.kind);
    }
}

unsafe extern "win64" fn gi_unregister_callback(
    _this: *mut GameInputObj,
    token: u64,
    _timeout_us: u64,
) -> u8 {
    let mut st = runtime_state().lock().unwrap();
    if let Some(entry) = st.callbacks.remove(&token) {
        log::debug!(
            "gameinput: UnregisterCallback(token={token}, kind={:?}, was_active={})",
            entry.kind,
            entry.active
        );
        1
    } else {
        0
    }
}

unsafe extern "win64" fn gi_create_dispatcher(
    _this: *mut GameInputObj,
    dispatcher_out: *mut *mut DispatcherObj,
) -> i32 {
    if dispatcher_out.is_null() {
        return E_POINTER;
    }
    let disp = Box::into_raw(Box::new(DispatcherObj {
        vtbl: &DISPATCHER_VTBL,
        refs: AtomicU32::new(1),
    }));
    std::ptr::write_unaligned(dispatcher_out, disp);
    S_OK
}

unsafe extern "win64" fn gi_create_aggregate_device(
    _this: *mut GameInputObj,
    input_kind: u32,
    device_out: *mut *mut DeviceObj,
) -> i32 {
    if device_out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(device_out, std::ptr::null_mut());
    let chosen = match input_kind {
        GAME_INPUT_KIND_KEYBOARD => Some(DeviceKind::Keyboard),
        GAME_INPUT_KIND_MOUSE => Some(DeviceKind::Mouse),
        GAME_INPUT_KIND_GAMEPAD | GAME_INPUT_KIND_CONTROLLER => {
            ensure_gamepad_started();
            (0..xinput1_4::XUSER_MAX_COUNT)
                .find(|&slot| xinput1_4::slot_state(slot).is_some())
                .map(DeviceKind::Gamepad)
        }
        _ => return E_INVALIDARG,
    };
    let Some(dk) = chosen else {
        return GAMEINPUT_E_DEVICE_NOT_FOUND;
    };
    let dev = device_ptr(dk);
    dev_add_ref(dev);
    std::ptr::write_unaligned(device_out, dev);
    S_OK
}

unsafe extern "win64" fn gi_find_device_from_id(
    _this: *mut GameInputObj,
    value: *const AppLocalDeviceId,
    device_out: *mut *mut DeviceObj,
) -> i32 {
    if device_out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(device_out, std::ptr::null_mut());
    if value.is_null() {
        return E_INVALIDARG;
    }
    let want = std::ptr::read_unaligned(value);
    for &raw in persistent_devices() {
        let dev = raw as *mut DeviceObj;
        if (*dev).info.device_id == want {
            if device_status_for((*dev).kind) == GAME_INPUT_DEVICE_NO_STATUS {
                return GAMEINPUT_E_DEVICE_NOT_FOUND;
            }
            dev_add_ref(dev);
            std::ptr::write_unaligned(device_out, dev);
            return S_OK;
        }
    }
    GAMEINPUT_E_DEVICE_NOT_FOUND
}

unsafe extern "win64" fn gi_find_device_from_object(
    _this: *mut GameInputObj,
    _value: *mut c_void,
    device_out: *mut *mut DeviceObj,
) -> i32 {
    if !device_out.is_null() {
        std::ptr::write_unaligned(device_out, std::ptr::null_mut());
    }
    GAMEINPUT_E_DEVICE_NOT_FOUND
}

unsafe extern "win64" fn gi_find_device_from_platform_handle(
    _this: *mut GameInputObj,
    _value: usize,
    device_out: *mut *mut DeviceObj,
) -> i32 {
    if !device_out.is_null() {
        std::ptr::write_unaligned(device_out, std::ptr::null_mut());
    }
    GAMEINPUT_E_DEVICE_NOT_FOUND
}

unsafe extern "win64" fn gi_find_device_from_platform_string(
    _this: *mut GameInputObj,
    _value: *const u16,
    device_out: *mut *mut DeviceObj,
) -> i32 {
    if !device_out.is_null() {
        std::ptr::write_unaligned(device_out, std::ptr::null_mut());
    }
    GAMEINPUT_E_DEVICE_NOT_FOUND
}

unsafe extern "win64" fn gi_enable_oem_device_support(
    _this: *mut GameInputObj,
    _vendor_id: u16,
    _product_id: u16,
    _interface_number: u8,
    _collection_number: u8,
) -> i32 {
    S_OK
}

unsafe extern "win64" fn gi_set_focus_policy(_this: *mut GameInputObj, policy: u32) {
    let mut st = runtime_state().lock().unwrap();
    let prev = st.focus_policy;
    st.focus_policy = policy;
    if prev != policy {
        log::debug!("gameinput: SetFocusPolicy {prev:#x} -> {policy:#x}");
    }
}

static GAMEINPUT_VTBL: [crate::Ptr; 22] = [
    crate::Ptr(gi_query_interface as *const ()),                       //  0 (+0x00) QueryInterface
    crate::Ptr(gi_add_ref as *const ()),                               //  1 (+0x08) AddRef
    crate::Ptr(gi_release as *const ()),                               //  2 (+0x10) Release
    crate::Ptr(gi_get_current_timestamp as *const ()),                 //  3 (+0x18) GetCurrentTimestamp
    crate::Ptr(gi_get_current_reading as *const ()),                   //  4 (+0x20) GetCurrentReading
    crate::Ptr(gi_get_next_reading as *const ()),                      //  5 (+0x28) GetNextReading
    crate::Ptr(gi_get_previous_reading as *const ()),                  //  6 (+0x30) GetPreviousReading
    crate::Ptr(gi_get_temporal_reading as *const ()),                  //  7 (+0x38) GetTemporalReading
    crate::Ptr(crate::bridge_entry!(gi_register_reading_callback)),    //  8 (+0x40) RegisterReadingCallback
    crate::Ptr(gi_register_device_callback as *const ()),              //  9 (+0x48) RegisterDeviceCallback
    crate::Ptr(gi_register_system_button_callback as *const ()),       // 10 (+0x50) RegisterSystemButtonCallback
    crate::Ptr(gi_register_keyboard_layout_callback as *const ()),     // 11 (+0x58) RegisterKeyboardLayoutCallback
    crate::Ptr(gi_stop_callback as *const ()),                         // 12 (+0x60) StopCallback
    crate::Ptr(gi_unregister_callback as *const ()),                   // 13 (+0x68) UnregisterCallback
    crate::Ptr(gi_create_dispatcher as *const ()),                     // 14 (+0x70) CreateDispatcher
    crate::Ptr(gi_create_aggregate_device as *const ()),               // 15 (+0x78) CreateAggregateDevice
    crate::Ptr(gi_find_device_from_id as *const ()),                   // 16 (+0x80) FindDeviceFromId
    crate::Ptr(gi_find_device_from_object as *const ()),               // 17 (+0x88) FindDeviceFromObject
    crate::Ptr(gi_find_device_from_platform_handle as *const ()),      // 18 (+0x90) FindDeviceFromPlatformHandle
    crate::Ptr(gi_find_device_from_platform_string as *const ()),      // 19 (+0x98) FindDeviceFromPlatformString
    crate::Ptr(gi_enable_oem_device_support as *const ()),             // 20 (+0xA0) EnableOemDeviceSupport
    crate::Ptr(gi_set_focus_policy as *const ()),                      // 21 (+0xA8) SetFocusPolicy
];

fn ensure_vtbls_registered() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        for p in GAMEINPUT_VTBL
            .iter()
            .chain(READING_VTBL.iter())
            .chain(DEVICE_VTBL.iter())
            .chain(DISPATCHER_VTBL.iter())
        {
            let addr = p.0 as usize;
            crate::register_native_va(addr as u64, addr);
        }
    });
}

// ---------------------------------------------------------------------------
// Exported DLL entry point
// ---------------------------------------------------------------------------

pub extern "win64" fn GameInputCreate(game_input_out: *mut usize) -> i32 {
    if game_input_out.is_null() {
        return E_INVALIDARG;
    }
    ensure_vtbls_registered();
    ensure_gamepad_started();
    let _ = persistent_devices();
    GAMEINPUT_ACTIVE.store(true, Ordering::Release);
    let now = current_timestamp_us();
    let kbd_snap = snapshot_keyboard();
    let mouse_snap = snapshot_mouse();
    let gamepad_snaps: [Option<GameInputGamepadState>; xinput1_4::XUSER_MAX_COUNT] =
        std::array::from_fn(|slot| {
            xinput1_4::slot_state(slot).map(|(pad, _)| xinput_to_gamepad_state(&pad))
        });
    {
        let mut st = runtime_state().lock().unwrap();
        if st.baseline_keyboard.is_none() {
            st.set_baseline(
                DeviceKind::Keyboard,
                GAME_INPUT_KIND_KEYBOARD,
                ReadingPayload::Keyboard(kbd_snap),
                now,
            );
        }
        if st.baseline_mouse.is_none() {
            st.set_baseline(
                DeviceKind::Mouse,
                GAME_INPUT_KIND_MOUSE,
                ReadingPayload::Mouse(mouse_snap),
                now,
            );
        }
        for (slot, maybe_gp) in gamepad_snaps.into_iter().enumerate() {
            if let Some(gp) = maybe_gp {
                let payload = ReadingPayload::Gamepad(gp);
                if st.baseline_gamepad[slot].is_none() {
                    st.set_baseline(DeviceKind::Gamepad(slot), GAME_INPUT_KIND_GAMEPAD, payload, now);
                } else if !st.history.iter().any(|e| e.device_kind == DeviceKind::Gamepad(slot))
                    && st.baseline_gamepad[slot]
                        .as_ref()
                        .is_some_and(|b| b.payload != payload)
                {
                    let _ = st.push_snapshot_at(
                        DeviceKind::Gamepad(slot),
                        GAME_INPUT_KIND_GAMEPAD,
                        payload,
                        now,
                    );
                }
            }
        }
    }
    let obj = Box::into_raw(Box::new(GameInputObj {
        vtbl: &GAMEINPUT_VTBL,
        refs: AtomicU32::new(1),
    })) as usize;
    unsafe {
        std::ptr::write_unaligned(game_input_out, obj);
    }
    S_OK
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user32::input::{MouseButton, NativeKey, NativeMouse};
    use crate::user32::POINT;

    static TEST_SERIAL: Mutex<()> = Mutex::new(());

    fn reset_test_state() {
        unsafe {
            xinput1_4::XInputEnable(1);
        }
        for slot in 0..xinput1_4::XUSER_MAX_COUNT {
            xinput1_4::backend_disconnect(slot);
        }
        xinput1_4::set_virtual_gamepad(false, xinput1_4::Gamepad::default());
        crate::platform::uikit::set_rumble_callback(None);
        MOUSE_WHEEL_X.store(0, Ordering::Relaxed);
        MOUSE_WHEEL_Y.store(0, Ordering::Relaxed);
        GAMEINPUT_ACTIVE.store(false, Ordering::Release);
        let mut st = runtime_state().lock().unwrap();
        st.history.clear();
        st.next_history_id = 0;
        st.seq_gamepad = [0; xinput1_4::XUSER_MAX_COUNT];
        st.seq_keyboard = 0;
        st.seq_mouse = 0;
        st.baseline_gamepad = std::array::from_fn(|_| None);
        st.baseline_keyboard = None;
        st.baseline_mouse = None;
        st.callbacks.clear();
    }

    unsafe fn vtbl_fn<F: Copy>(ptr: *const crate::Ptr, idx: usize) -> F {
        std::mem::transmute_copy(&(*ptr.add(idx)).0)
    }

    #[test]
    fn game_input_create_qi_and_refcount_lifecycle() {
        let _g = TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        reset_test_state();
        assert_eq!(GameInputCreate(std::ptr::null_mut()), E_INVALIDARG);

        let mut gi_raw = 0usize;
        assert_eq!(GameInputCreate(&mut gi_raw), S_OK);
        assert_ne!(gi_raw, 0);
        let gi = gi_raw as *mut GameInputObj;

        unsafe {
            let vtbl = (*gi).vtbl as *const crate::Ptr;
            let qi: unsafe extern "win64" fn(*mut GameInputObj, *const Guid, *mut *mut c_void) -> i32 = vtbl_fn(vtbl, 0);
            let add_ref: unsafe extern "win64" fn(*mut GameInputObj) -> u32 = vtbl_fn(vtbl, 1);
            let release: unsafe extern "win64" fn(*mut GameInputObj) -> u32 = vtbl_fn(vtbl, 2);

            let mut queried: *mut c_void = std::ptr::null_mut();
            assert_eq!(qi(gi, &IID_IGAMEINPUT, &mut queried), S_OK);
            assert_eq!(queried as *mut GameInputObj, gi);

            let mut bad: *mut c_void = 1 as *mut c_void;
            assert_eq!(qi(gi, &IID_IGAMEINPUT_DEVICE, &mut bad), E_NOINTERFACE);
            assert!(bad.is_null());

            assert_eq!(add_ref(gi), 3);
            assert_eq!(release(gi), 2);
            assert_eq!(release(gi), 1);
            assert_eq!(release(gi), 0);
        }
    }

    #[test]
    fn get_current_timestamp_is_monotonic_microseconds() {
        let _g = TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let mut gi_raw = 0usize;
        assert_eq!(GameInputCreate(&mut gi_raw), S_OK);
        let gi = gi_raw as *mut GameInputObj;
        unsafe {
            let vtbl = (*gi).vtbl as *const crate::Ptr;
            let get_ts: unsafe extern "win64" fn(*mut GameInputObj) -> u64 = vtbl_fn(vtbl, 3);
            let release: unsafe extern "win64" fn(*mut GameInputObj) -> u32 = vtbl_fn(vtbl, 2);

            let t1 = get_ts(gi);
            assert!(t1 > 0);
            std::thread::sleep(std::time::Duration::from_millis(2));
            let t2 = get_ts(gi);
            assert!(t2 > t1, "expected monotonic timestamp increase: t1={t1}, t2={t2}");
            assert_eq!(release(gi), 0);
        }
    }

    #[test]
    fn gamepad_button_axis_and_trigger_translation_math() {
        let pad = xinput1_4::Gamepad {
            buttons: xinput1_4::btn::START
                | xinput1_4::btn::BACK
                | xinput1_4::btn::A
                | xinput1_4::btn::B
                | xinput1_4::btn::X
                | xinput1_4::btn::Y
                | xinput1_4::btn::DPAD_UP
                | xinput1_4::btn::DPAD_DOWN
                | xinput1_4::btn::DPAD_LEFT
                | xinput1_4::btn::DPAD_RIGHT
                | xinput1_4::btn::LEFT_SHOULDER
                | xinput1_4::btn::RIGHT_SHOULDER
                | xinput1_4::btn::LEFT_THUMB
                | xinput1_4::btn::RIGHT_THUMB
                | xinput1_4::btn::GUIDE,
            left_trigger: 255,
            right_trigger: 128,
            thumb_lx: -32768,
            thumb_ly: 32767,
            thumb_rx: -16384,
            thumb_ry: 0,
        };
        let st = xinput_to_gamepad_state(&pad);
        assert_eq!(st.buttons, GAMEPAD_STANDARD_LAYOUT, "all 14 buttons mapped; Guide excluded");
        assert!((st.left_trigger - 1.0).abs() < 1e-6);
        assert!((st.right_trigger - (128.0 / 255.0)).abs() < 1e-6);
        assert!((st.left_thumbstick_x + 1.0).abs() < 1e-6);
        assert!((st.left_thumbstick_y - 1.0).abs() < 1e-6);
        assert!((st.right_thumbstick_x + 0.5).abs() < 1e-6);
        assert_eq!(st.right_thumbstick_y, 0.0);

        let neutral = xinput_to_gamepad_state(&xinput1_4::Gamepad::default());
        assert_eq!(neutral, GameInputGamepadState::default());

        assert_eq!(rumble_params_to_xinput(GameInputRumbleParams {
            low_frequency: 0.5,
            high_frequency: 1.0,
            left_trigger: 0.0,
            right_trigger: 0.0,
        }), (32768, 65535));
        assert_eq!(rumble_params_to_xinput(GameInputRumbleParams {
            low_frequency: 0.0,
            high_frequency: 0.0,
            left_trigger: 0.75,
            right_trigger: 0.25,
        }), (49151, 16384));
        assert_eq!(rumble_params_to_xinput(GameInputRumbleParams {
            low_frequency: f32::NAN,
            high_frequency: 2.5,
            left_trigger: -1.0,
            right_trigger: f32::INFINITY,
        }), (0, 65535));
    }

    #[test]
    fn keyboard_and_mouse_readings_track_live_user32_state() {
        let _g = TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let _u = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        reset_test_state();

        // Press 'W' (mac 0x0D) and Left Shift (mac 0x38).
        crate::user32::input::native_key(NativeKey { mac: 0x0D, down: true, repeat: false, chars: vec![b'w' as u16] });
        crate::user32::input::native_key(NativeKey { mac: 0x38, down: true, repeat: false, chars: Vec::new() });

        // Move mouse & press Left + Right buttons + scroll wheel.
        unsafe {
            let _ = crate::user32::cursor::ClipCursor(std::ptr::null());
        }
        crate::user32::cursor::set_cached_pos(POINT { x: 640, y: 360 });
        crate::user32::input::native_mouse(NativeMouse::Button {
            hwnd: 0,
            client: None,
            screen: POINT { x: 640, y: 360 },
            button: MouseButton::Left,
            down: true,
            clicks: 1,
        });
        crate::user32::input::native_mouse(NativeMouse::Button {
            hwnd: 0,
            client: None,
            screen: POINT { x: 640, y: 360 },
            button: MouseButton::Right,
            down: true,
            clicks: 1,
        });
        record_mouse_wheel(120, -240);

        let mut gi_raw = 0usize;
        assert_eq!(GameInputCreate(&mut gi_raw), S_OK);
        let gi = gi_raw as *mut GameInputObj;

        unsafe {
            let vtbl = (*gi).vtbl as *const crate::Ptr;
            let get_cur: unsafe extern "win64" fn(*mut GameInputObj, u32, *mut DeviceObj, *mut *mut ReadingObj) -> i32 = vtbl_fn(vtbl, 4);
            let gi_release: unsafe extern "win64" fn(*mut GameInputObj) -> u32 = vtbl_fn(vtbl, 2);

            // Keyboard reading
            let mut kbd_rd: *mut ReadingObj = std::ptr::null_mut();
            assert_eq!(get_cur(gi, GAME_INPUT_KIND_KEYBOARD, std::ptr::null_mut(), &mut kbd_rd), S_OK);
            assert!(!kbd_rd.is_null());
            let rvt = (*kbd_rd).vtbl as *const crate::Ptr;
            let get_kind: unsafe extern "win64" fn(*mut ReadingObj) -> u32 = vtbl_fn(rvt, 3);
            let get_seq: unsafe extern "win64" fn(*mut ReadingObj, u32) -> u64 = vtbl_fn(rvt, 4);
            let get_ts: unsafe extern "win64" fn(*mut ReadingObj) -> u64 = vtbl_fn(rvt, 5);
            let get_dev: unsafe extern "win64" fn(*mut ReadingObj, *mut *mut DeviceObj) = vtbl_fn(rvt, 6);
            let get_key_count: unsafe extern "win64" fn(*mut ReadingObj) -> u32 = vtbl_fn(rvt, 14);
            let get_key_state: unsafe extern "win64" fn(*mut ReadingObj, u32, *mut GameInputKeyState) -> u32 = vtbl_fn(rvt, 15);
            let rd_rel: unsafe extern "win64" fn(*mut ReadingObj) -> u32 = vtbl_fn(rvt, 2);

            assert_eq!(get_kind(kbd_rd), GAME_INPUT_KIND_KEYBOARD);
            assert!(get_seq(kbd_rd, GAME_INPUT_KIND_KEYBOARD) >= 1);
            assert!(get_ts(kbd_rd) > 0);
            let count = get_key_count(kbd_rd);
            assert!(count >= 2, "expected at least W and LShift pressed, got {count}");
            let mut keys = vec![GameInputKeyState::default(); count as usize];
            assert_eq!(get_key_state(kbd_rd, count, keys.as_mut_ptr()), count);
            let w_key = keys.iter().find(|k| k.virtual_key == b'W').expect("W key pressed");
            assert_eq!(w_key.scan_code, 0x11);
            let expected_w = if crate::user32::input::caps_on() { b'w' as u32 } else { b'W' as u32 };
            assert_eq!(w_key.code_point, expected_w, "Shift+W respects shift/caps state");
            assert_eq!(w_key.is_dead_key, 0);
            let lshift_key = keys.iter().find(|k| k.virtual_key == keymap::VK_LSHIFT).expect("LShift pressed");
            assert_eq!(lshift_key.scan_code, 0x2A);
            assert!(keys.iter().all(|k| k.virtual_key != keymap::VK_SHIFT), "generic VK_SHIFT not duplicated");

            // Keyboard device info
            let mut kdev: *mut DeviceObj = std::ptr::null_mut();
            get_dev(kbd_rd, &mut kdev);
            assert!(!kdev.is_null());
            let dvt = (*kdev).vtbl as *const crate::Ptr;
            let get_info: unsafe extern "win64" fn(*mut DeviceObj) -> *const GameInputDeviceInfo = vtbl_fn(dvt, 3);
            let get_status: unsafe extern "win64" fn(*mut DeviceObj) -> u32 = vtbl_fn(dvt, 4);
            let dev_rel: unsafe extern "win64" fn(*mut DeviceObj) -> u32 = vtbl_fn(dvt, 2);
            let kinfo = &*get_info(kdev);
            assert_eq!(kinfo.supported_input, GAME_INPUT_KIND_KEYBOARD);
            assert_eq!(kinfo.device_family, GAME_INPUT_FAMILY_HID);
            assert!(!kinfo.keyboard_info.is_null());
            assert!(kinfo.mouse_info.is_null());
            assert!(kinfo.gamepad_info.is_null());
            assert_eq!((*kinfo.keyboard_info).key_count, 104);
            assert_eq!(get_status(kdev), GAME_INPUT_DEVICE_CONNECTED | GAME_INPUT_DEVICE_INPUT_ENABLED);
            let _ = dev_rel(kdev);
            assert_eq!(rd_rel(kbd_rd), 0);

            // Mouse reading
            let mut m_rd: *mut ReadingObj = std::ptr::null_mut();
            assert_eq!(get_cur(gi, GAME_INPUT_KIND_MOUSE, std::ptr::null_mut(), &mut m_rd), S_OK);
            assert!(!m_rd.is_null());
            let mrvt = (*m_rd).vtbl as *const crate::Ptr;
            let get_mouse: unsafe extern "win64" fn(*mut ReadingObj, *mut GameInputMouseState) -> u8 = vtbl_fn(mrvt, 16);
            let m_kind: unsafe extern "win64" fn(*mut ReadingObj) -> u32 = vtbl_fn(mrvt, 3);
            let m_dev: unsafe extern "win64" fn(*mut ReadingObj, *mut *mut DeviceObj) = vtbl_fn(mrvt, 6);
            let m_rel: unsafe extern "win64" fn(*mut ReadingObj) -> u32 = vtbl_fn(mrvt, 2);
            assert_eq!(m_kind(m_rd), GAME_INPUT_KIND_MOUSE);

            let mut ms = GameInputMouseState::default();
            assert_eq!(get_mouse(m_rd, &mut ms), 1);
            assert_eq!(ms.buttons, GAME_INPUT_MOUSE_LEFT_BUTTON | GAME_INPUT_MOUSE_RIGHT_BUTTON);
            assert_eq!((ms.position_x, ms.position_y), (640, 360));
            assert_eq!((ms.wheel_x, ms.wheel_y), (120, -240));

            let mut mdev: *mut DeviceObj = std::ptr::null_mut();
            m_dev(m_rd, &mut mdev);
            assert!(!mdev.is_null());
            let mdvt = (*mdev).vtbl as *const crate::Ptr;
            let m_get_info: unsafe extern "win64" fn(*mut DeviceObj) -> *const GameInputDeviceInfo = vtbl_fn(mdvt, 3);
            let m_dev_rel: unsafe extern "win64" fn(*mut DeviceObj) -> u32 = vtbl_fn(mdvt, 2);
            let minfo = &*m_get_info(mdev);
            assert_eq!(minfo.supported_input, GAME_INPUT_KIND_MOUSE);
            assert!(!minfo.mouse_info.is_null());
            assert_eq!((*minfo.mouse_info).has_wheel_y, 1);
            let _ = m_dev_rel(mdev);
            assert_eq!(m_rel(m_rd), 0);

            // Unsupported input kinds honestly return GAMEINPUT_E_READING_NOT_FOUND
            let mut unsupported: *mut ReadingObj = 1 as *mut ReadingObj;
            assert_eq!(get_cur(gi, GAME_INPUT_KIND_RACING_WHEEL, std::ptr::null_mut(), &mut unsupported), GAMEINPUT_E_READING_NOT_FOUND);
            assert!(unsupported.is_null());
            assert_eq!(get_cur(gi, GAME_INPUT_KIND_FLIGHT_STICK, std::ptr::null_mut(), &mut unsupported), GAMEINPUT_E_READING_NOT_FOUND);
            assert!(unsupported.is_null());

            assert_eq!(gi_release(gi), 0);
        }

        // Release synthetic keys & mouse buttons cleanly
        crate::user32::input::native_key(NativeKey { mac: 0x0D, down: false, repeat: false, chars: Vec::new() });
        crate::user32::input::native_key(NativeKey { mac: 0x38, down: false, repeat: false, chars: Vec::new() });
        crate::user32::input::native_mouse(NativeMouse::Button {
            hwnd: 0,
            client: None,
            screen: POINT { x: 0, y: 0 },
            button: MouseButton::Left,
            down: false,
            clicks: 1,
        });
        crate::user32::input::native_mouse(NativeMouse::Button {
            hwnd: 0,
            client: None,
            screen: POINT { x: 0, y: 0 },
            button: MouseButton::Right,
            down: false,
            clicks: 1,
        });
        reset_test_state();
    }

    #[test]
    fn gamepad_live_reading_history_device_info_and_rumble() {
        static RUMBLE_LOG: Mutex<Vec<(u16, u16)>> = Mutex::new(Vec::new());
        extern "C" fn capture_rumble(l: u16, r: u16) {
            RUMBLE_LOG.lock().unwrap().push((l, r));
        }

        let _g = TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        reset_test_state();
        RUMBLE_LOG.lock().unwrap().clear();

        let mut gi_raw = 0usize;
        assert_eq!(GameInputCreate(&mut gi_raw), S_OK);
        let gi = gi_raw as *mut GameInputObj;

        unsafe {
            let vtbl = (*gi).vtbl as *const crate::Ptr;
            let get_cur: unsafe extern "win64" fn(*mut GameInputObj, u32, *mut DeviceObj, *mut *mut ReadingObj) -> i32 = vtbl_fn(vtbl, 4);
            let get_next: unsafe extern "win64" fn(*mut GameInputObj, *mut ReadingObj, u32, *mut DeviceObj, *mut *mut ReadingObj) -> i32 = vtbl_fn(vtbl, 5);
            let get_prev: unsafe extern "win64" fn(*mut GameInputObj, *mut ReadingObj, u32, *mut DeviceObj, *mut *mut ReadingObj) -> i32 = vtbl_fn(vtbl, 6);
            let find_by_id: unsafe extern "win64" fn(*mut GameInputObj, *const AppLocalDeviceId, *mut *mut DeviceObj) -> i32 = vtbl_fn(vtbl, 16);
            let gi_release: unsafe extern "win64" fn(*mut GameInputObj) -> u32 = vtbl_fn(vtbl, 2);

            // Disconnected gamepad -> GAMEINPUT_E_READING_NOT_FOUND
            let mut r0: *mut ReadingObj = std::ptr::null_mut();
            assert_eq!(get_cur(gi, GAME_INPUT_KIND_GAMEPAD, std::ptr::null_mut(), &mut r0), GAMEINPUT_E_READING_NOT_FOUND);
            assert!(r0.is_null());

            // Connect slot 0 with host rumble callback so SetRumbleState is observably verified end-to-end.
            crate::platform::uikit::set_rumble_callback(Some(capture_rumble));
            xinput1_4::backend_connect(
                0,
                xinput1_4::DeviceInfo {
                    name: "Xbox Test Pad".into(),
                    wireless: true,
                    rumble: false, // routes slot 0 through host rumble callback for test capture
                    battery: Some(0.5),
                },
            );

            // Reading 1: A + DPadUp + LX = -0.5 + LT = 1.0
            let pad1 = xinput1_4::GcPad {
                a: true,
                dpad_up: true,
                lx: -0.5,
                left_trigger: 1.0,
                ..Default::default()
            }
            .to_xinput();
            xinput1_4::backend_update(0, pad1);

            let mut r1: *mut ReadingObj = std::ptr::null_mut();
            assert_eq!(get_cur(gi, GAME_INPUT_KIND_GAMEPAD, std::ptr::null_mut(), &mut r1), S_OK);
            assert!(!r1.is_null());

            let rvt = (*r1).vtbl as *const crate::Ptr;
            let get_kind: unsafe extern "win64" fn(*mut ReadingObj) -> u32 = vtbl_fn(rvt, 3);
            let get_seq: unsafe extern "win64" fn(*mut ReadingObj, u32) -> u64 = vtbl_fn(rvt, 4);
            let get_dev: unsafe extern "win64" fn(*mut ReadingObj, *mut *mut DeviceObj) = vtbl_fn(rvt, 6);
            let get_switch: unsafe extern "win64" fn(*mut ReadingObj, u32, *mut u32) -> u32 = vtbl_fn(rvt, 13);
            let get_gp: unsafe extern "win64" fn(*mut ReadingObj, *mut GameInputGamepadState) -> u8 = vtbl_fn(rvt, 22);
            let rd_add: unsafe extern "win64" fn(*mut ReadingObj) -> u32 = vtbl_fn(rvt, 1);
            let rd_rel: unsafe extern "win64" fn(*mut ReadingObj) -> u32 = vtbl_fn(rvt, 2);

            assert_eq!(get_kind(r1), GAME_INPUT_KIND_GAMEPAD);
            let mut gp1 = GameInputGamepadState::default();
            assert_eq!(get_gp(r1, &mut gp1), 1);
            assert_eq!(gp1.buttons, GAME_INPUT_GAMEPAD_A | GAME_INPUT_GAMEPAD_DPAD_UP);
            assert!((gp1.left_thumbstick_x + 0.5).abs() < 1e-4);
            assert!((gp1.left_trigger - 1.0).abs() < 1e-6);

            let mut sw = 0u32;
            assert_eq!(get_switch(r1, 1, &mut sw), 1);
            assert_eq!(sw, GAME_INPUT_SWITCH_UP);

            // Inspect IGameInputDevice from r1
            let mut gdev: *mut DeviceObj = std::ptr::null_mut();
            get_dev(r1, &mut gdev);
            assert!(!gdev.is_null());
            let dvt = (*gdev).vtbl as *const crate::Ptr;
            let dev_add: unsafe extern "win64" fn(*mut DeviceObj) -> u32 = vtbl_fn(dvt, 1);
            let dev_rel: unsafe extern "win64" fn(*mut DeviceObj) -> u32 = vtbl_fn(dvt, 2);
            let get_info: unsafe extern "win64" fn(*mut DeviceObj) -> *const GameInputDeviceInfo = vtbl_fn(dvt, 3);
            let get_status: unsafe extern "win64" fn(*mut DeviceObj) -> u32 = vtbl_fn(dvt, 4);
            let get_bat: unsafe extern "win64" fn(*mut DeviceObj, *mut GameInputBatteryState) = vtbl_fn(dvt, 5);
            let set_rumble: unsafe extern "win64" fn(*mut DeviceObj, *const GameInputRumbleParams) = vtbl_fn(dvt, 10);

            let info = &*get_info(gdev);
            assert_eq!(info.info_size as usize, std::mem::size_of::<GameInputDeviceInfo>());
            assert_eq!(info.vendor_id, 0x045E);
            assert_eq!(info.product_id, 0x02EA);
            assert_eq!(info.device_family, GAME_INPUT_FAMILY_XBOX_ONE);
            assert_ne!(info.supported_input & GAME_INPUT_KIND_GAMEPAD, 0);
            assert!(!info.gamepad_info.is_null());
            assert_eq!((*info.gamepad_info).supported_layout, GAMEPAD_STANDARD_LAYOUT);
            assert_eq!(get_status(gdev), GAME_INPUT_DEVICE_CONNECTED | GAME_INPUT_DEVICE_INPUT_ENABLED);

            // Battery state reflects wireless pad level (0.5 -> MEDIUM -> 0.65)
            let mut bat = GameInputBatteryState::default();
            get_bat(gdev, &mut bat);
            assert_eq!(bat.status, GAME_INPUT_BATTERY_DISCHARGING);
            assert!((bat.remaining_capacity - 0.65).abs() < 1e-6);

            // FindDeviceFromId locates the exact same gamepad device
            let mut found_dev: *mut DeviceObj = std::ptr::null_mut();
            assert_eq!(find_by_id(gi, &info.device_id, &mut found_dev), S_OK);
            assert_eq!(found_dev, gdev);
            let _ = dev_rel(found_dev);

            // SetRumbleState forwards through xinput1_4 -> host rumble callback, and nullptr stops rumble
            let rumble = GameInputRumbleParams {
                low_frequency: 0.5,
                high_frequency: 0.25,
                left_trigger: 0.0,
                right_trigger: 0.0,
            };
            set_rumble(gdev, &rumble);
            set_rumble(gdev, std::ptr::null());
            assert_eq!(*RUMBLE_LOG.lock().unwrap(), vec![(32768, 16384), (0, 0)]);

            // Verify bridged float method slot matches bridge_entry! and updates gain
            assert_eq!((*(*gdev).vtbl)[8].0, crate::bridge_entry!(dev_set_force_feedback_motor_gain));
            dev_set_force_feedback_motor_gain(gdev, 0, 0.75);
            assert!((*(*gdev).ffb_gain.lock().unwrap() - 0.75).abs() < 1e-6);

            // Reading 2: state changes -> B + right trigger -> ring history navigation
            let pad2 = xinput1_4::GcPad {
                b: true,
                right_trigger: 0.5,
                ry: -1.0,
                ..Default::default()
            }
            .to_xinput();
            xinput1_4::backend_update(0, pad2);

            let mut r2: *mut ReadingObj = std::ptr::null_mut();
            assert_eq!(get_cur(gi, GAME_INPUT_KIND_GAMEPAD, gdev, &mut r2), S_OK);
            assert!(get_seq(r2, GAME_INPUT_KIND_GAMEPAD) > get_seq(r1, GAME_INPUT_KIND_GAMEPAD));

            // Walk backward: GetPreviousReading(r2) -> r1, then boundary -> NOT_FOUND
            let mut prev: *mut ReadingObj = std::ptr::null_mut();
            assert_eq!(get_prev(gi, r2, GAME_INPUT_KIND_GAMEPAD, gdev, &mut prev), S_OK);
            let mut gp_prev = GameInputGamepadState::default();
            assert_eq!(get_gp(prev, &mut gp_prev), 1);
            assert_eq!(gp_prev.buttons, GAME_INPUT_GAMEPAD_A | GAME_INPUT_GAMEPAD_DPAD_UP);
            let mut before_r1: *mut ReadingObj = std::ptr::null_mut();
            assert_eq!(get_prev(gi, prev, GAME_INPUT_KIND_GAMEPAD, gdev, &mut before_r1), GAMEINPUT_E_READING_NOT_FOUND);
            assert!(before_r1.is_null());

            // Walk forward: GetNextReading(r1) -> r2, then boundary -> NOT_FOUND
            let mut next: *mut ReadingObj = std::ptr::null_mut();
            assert_eq!(get_next(gi, r1, GAME_INPUT_KIND_GAMEPAD, gdev, &mut next), S_OK);
            let mut gp_next = GameInputGamepadState::default();
            assert_eq!(get_gp(next, &mut gp_next), 1);
            assert_eq!(gp_next.buttons, GAME_INPUT_GAMEPAD_B);
            assert!((gp_next.right_thumbstick_y + 1.0).abs() < 1e-6);
            let mut after_r2: *mut ReadingObj = std::ptr::null_mut();
            assert_eq!(get_next(gi, next, GAME_INPUT_KIND_GAMEPAD, gdev, &mut after_r2), GAMEINPUT_E_READING_NOT_FOUND);
            assert!(after_r2.is_null());

            // Device & reading refcounts
            assert_eq!(rd_add(r1), 2);
            assert_eq!(rd_rel(r1), 1);
            assert_eq!(rd_rel(r1), 0);
            assert_eq!(rd_rel(r2), 0);
            assert_eq!(rd_rel(prev), 0);
            assert_eq!(rd_rel(next), 0);

            // Disconnect slot 0 -> GetDeviceStatus == 0 and GetCurrentReading fails honestly
            xinput1_4::backend_disconnect(0);
            assert_eq!(get_status(gdev), GAME_INPUT_DEVICE_NO_STATUS);
            let mut r_disc: *mut ReadingObj = std::ptr::null_mut();
            assert_eq!(get_cur(gi, GAME_INPUT_KIND_GAMEPAD, gdev, &mut r_disc), GAMEINPUT_E_READING_NOT_FOUND);

            let c = dev_add(gdev);
            assert_eq!(dev_rel(gdev), c - 1);
            let _ = dev_rel(gdev);
            assert_eq!(gi_release(gi), 0);
        }

        reset_test_state();
    }

    unsafe extern "win64" fn record_device_cb(
        token: u64,
        ctx: usize,
        device: *mut DeviceObj,
        timestamp: u64,
        current_status: u32,
        previous_status: u32,
    ) {
        let log = &mut *(ctx as *mut Vec<(u64, *mut DeviceObj, u64, u32, u32)>);
        log.push((token, device, timestamp, current_status, previous_status));
    }

    #[test]
    fn callbacks_enumeration_and_aarch64_float_bridge() {
        let _g = TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        reset_test_state();

        xinput1_4::backend_connect(
            1,
            xinput1_4::DeviceInfo {
                name: "Slot 1 Pad".into(),
                wireless: false,
                rumble: false,
                battery: None,
            },
        );

        let mut gi_raw = 0usize;
        assert_eq!(GameInputCreate(&mut gi_raw), S_OK);
        let gi = gi_raw as *mut GameInputObj;

        unsafe {
            let vtbl = (*gi).vtbl as *const crate::Ptr;
            let reg_dev: unsafe extern "win64" fn(
                *mut GameInputObj,
                *mut DeviceObj,
                u32,
                u32,
                u32,
                usize,
                usize,
                *mut u64,
            ) -> i32 = vtbl_fn(vtbl, 9);
            let stop_cb: unsafe extern "win64" fn(*mut GameInputObj, u64) = vtbl_fn(vtbl, 12);
            let unreg_cb: unsafe extern "win64" fn(*mut GameInputObj, u64, u64) -> u8 = vtbl_fn(vtbl, 13);
            let create_disp: unsafe extern "win64" fn(*mut GameInputObj, *mut *mut DispatcherObj) -> i32 = vtbl_fn(vtbl, 14);
            let gi_release: unsafe extern "win64" fn(*mut GameInputObj) -> u32 = vtbl_fn(vtbl, 2);

            let mut seen: Vec<(u64, *mut DeviceObj, u64, u32, u32)> = Vec::new();
            let mut dev_token = 0u64;
            assert_eq!(
                reg_dev(
                    gi,
                    std::ptr::null_mut(),
                    GAME_INPUT_KIND_GAMEPAD | GAME_INPUT_KIND_KEYBOARD | GAME_INPUT_KIND_MOUSE,
                    GAME_INPUT_DEVICE_CONNECTED,
                    GAME_INPUT_BLOCKING_ENUMERATION,
                    &mut seen as *mut _ as usize,
                    record_device_cb as *const () as usize,
                    &mut dev_token,
                ),
                S_OK
            );
            assert_ne!(dev_token, 0);
            // Keyboard + Mouse + connected slot 1 gamepad = 3 connected devices enumerated
            assert_eq!(seen.len(), 3);
            for &(tok, dev, ts, cur, prev) in &seen {
                assert_eq!(tok, dev_token);
                assert!(!dev.is_null());
                assert!(ts > 0);
                assert_eq!(cur, GAME_INPUT_DEVICE_CONNECTED | GAME_INPUT_DEVICE_INPUT_ENABLED);
                assert_eq!(prev, GAME_INPUT_DEVICE_NO_STATUS);
            }

            // RegisterReadingCallback (direct call + vtable bridge verification)
            assert_eq!((*(*gi).vtbl)[8].0, crate::bridge_entry!(gi_register_reading_callback));
            let mut rd_token = 0u64;
            assert_eq!(
                gi_register_reading_callback(
                    gi,
                    std::ptr::null_mut(),
                    GAME_INPUT_KIND_GAMEPAD,
                    0.15,
                    0,
                    record_device_cb as *const () as usize,
                    &mut rd_token,
                ),
                S_OK
            );
            assert_ne!(rd_token, 0);
            assert_ne!(rd_token, dev_token);

            // On aarch64, verify the generated win64_bridge! entry unpacks float slot 4 + stack slots 5..7.
            #[cfg(target_arch = "aarch64")]
            {
                let mut bridge_tok = 0u64;
                let hr = gi_register_reading_callback::bridge(
                    gi as u64,
                    0,
                    GAME_INPUT_KIND_GAMEPAD as u64,
                    0,
                    0,
                    record_device_cb as *const () as u64,
                    &mut bridge_tok as *mut u64 as u64,
                    0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0.0,
                    0.0,
                    0.0,
                    f64::from_bits(0.25f32.to_bits() as u64),
                );
                assert_eq!(hr, S_OK);
                assert_ne!(bridge_tok, 0);
                assert_eq!(unreg_cb(gi, bridge_tok, 0), 1);
            }

            stop_cb(gi, dev_token);
            assert_eq!(unreg_cb(gi, dev_token, 0), 1);
            assert_eq!(unreg_cb(gi, dev_token, 0), 0);
            assert_eq!(unreg_cb(gi, rd_token, 0), 1);

            // Dispatcher lifecycle
            let mut disp: *mut DispatcherObj = std::ptr::null_mut();
            assert_eq!(create_disp(gi, &mut disp), S_OK);
            assert!(!disp.is_null());
            let disp_vt = (*disp).vtbl as *const crate::Ptr;
            let disp_dispatch_fn: unsafe extern "win64" fn(*mut DispatcherObj, u64) -> u8 = vtbl_fn(disp_vt, 3);
            let disp_rel: unsafe extern "win64" fn(*mut DispatcherObj) -> u32 = vtbl_fn(disp_vt, 2);
            assert_eq!(disp_dispatch_fn(disp, 1000), 0);
            assert_eq!(disp_rel(disp), 0);

            assert_eq!(gi_release(gi), 0);
        }

        reset_test_state();
    }

    #[test]
    fn get_temporal_reading_finds_matching_history_snapshot() {
        let _g = TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let _u = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        reset_test_state();

        unsafe {
            let t_before_connect = current_timestamp_us();
            std::thread::sleep(std::time::Duration::from_millis(2));
            xinput1_4::backend_connect(
                0,
                xinput1_4::DeviceInfo { name: "TemporalPad".into(), wireless: false, rumble: false, battery: None },
            );
            let mut gi_raw = 0usize;
            assert_eq!(GameInputCreate(&mut gi_raw), S_OK);
            let gi = gi_raw as *mut GameInputObj;
            let dev0 = device_ptr(DeviceKind::Gamepad(0));
            let kdev = device_ptr(DeviceKind::Keyboard);

            // Null device -> E_INVALIDARG.
            let mut out: *mut ReadingObj = std::ptr::null_mut();
            assert_eq!(gi_get_temporal_reading(gi, 1000, std::ptr::null_mut(), &mut out), E_INVALIDARG);

            // Timestamp before device existed -> GAMEINPUT_E_READING_NOT_FOUND.
            assert_eq!(gi_get_temporal_reading(gi, t_before_connect, dev0, &mut out), GAMEINPUT_E_READING_NOT_FOUND);

            // Save t0 BEFORE any reading query, then press A without polling first.
            // Historical lookup at t0 must still resolve the initial idle baseline captured
            // at connection time (never returning GAMEINPUT_E_READING_NOT_FOUND).
            let t0 = gi_get_current_timestamp(gi);
            let mut gs = GameInputGamepadState::default();

            // Input transitions recorded independently of GetCurrentReading must be returned by
            // exact GetCurrentTimestamp() -> GetTemporalReading() with zero future offset.
            std::thread::sleep(std::time::Duration::from_millis(2));
            xinput1_4::backend_update(0, xinput1_4::Gamepad { buttons: xinput1_4::btn::A, ..Default::default() });
            let t1 = gi_get_current_timestamp(gi);
            assert!(t1 > t0);

            // First ever reading query is for t0 (pre-press) AFTER A was already pressed:
            assert_eq!(gi_get_temporal_reading(gi, t0, dev0, &mut out), S_OK);
            assert_eq!(rd_get_gamepad_state(out, &mut gs), 1);
            assert_eq!(gs.buttons, 0, "baseline at t0 must remain queryable after later input transitions");
            let mut prev_from_base: *mut ReadingObj = std::ptr::null_mut();
            assert_eq!(
                gi_get_previous_reading(gi, out, GAME_INPUT_KIND_GAMEPAD, dev0, &mut prev_from_base),
                GAMEINPUT_E_READING_NOT_FOUND
            );
            let mut next_from_base: *mut ReadingObj = std::ptr::null_mut();
            assert_eq!(
                gi_get_next_reading(gi, out, GAME_INPUT_KIND_GAMEPAD, dev0, &mut next_from_base),
                S_OK
            );
            assert_eq!(rd_get_gamepad_state(next_from_base, &mut gs), 1);
            assert_eq!(gs.buttons, GAME_INPUT_GAMEPAD_A);
            assert_eq!(rd_release(next_from_base), 0);
            assert_eq!(rd_release(out), 0);

            assert_eq!(gi_get_temporal_reading(gi, t1, dev0, &mut out), S_OK);
            assert_eq!(rd_get_gamepad_state(out, &mut gs), 1);
            assert_eq!(gs.buttons, GAME_INPUT_GAMEPAD_A);
            assert_eq!(rd_release(out), 0);

            std::thread::sleep(std::time::Duration::from_millis(2));
            xinput1_4::backend_update(0, xinput1_4::Gamepad { buttons: xinput1_4::btn::B, ..Default::default() });
            let t2 = gi_get_current_timestamp(gi);
            assert!(t2 > t1);
            assert_eq!(gi_get_temporal_reading(gi, t2, dev0, &mut out), S_OK);
            assert_eq!(rd_get_gamepad_state(out, &mut gs), 1);
            assert_eq!(gs.buttons, GAME_INPUT_GAMEPAD_B);
            assert_eq!(rd_release(out), 0);

            std::thread::sleep(std::time::Duration::from_millis(2));
            xinput1_4::backend_update(0, xinput1_4::Gamepad { buttons: xinput1_4::btn::X, ..Default::default() });
            let t3 = gi_get_current_timestamp(gi);
            assert!(t3 > t2);
            assert_eq!(gi_get_temporal_reading(gi, t3, dev0, &mut out), S_OK);
            assert_eq!(rd_get_gamepad_state(out, &mut gs), 1);
            assert_eq!(gs.buttons, GAME_INPUT_GAMEPAD_X);
            assert_eq!(rd_release(out), 0);

            // Historical queries at t1 and t2 still resolve to A and B respectively.
            assert_eq!(gi_get_temporal_reading(gi, t1, dev0, &mut out), S_OK);
            assert_eq!(rd_get_gamepad_state(out, &mut gs), 1);
            assert_eq!(gs.buttons, GAME_INPUT_GAMEPAD_A);
            assert_eq!(rd_release(out), 0);

            assert_eq!(gi_get_temporal_reading(gi, t2, dev0, &mut out), S_OK);
            assert_eq!(rd_get_gamepad_state(out, &mut gs), 1);
            assert_eq!(gs.buttons, GAME_INPUT_GAMEPAD_B);
            assert_eq!(rd_release(out), 0);

            // Keyboard transition via user32::input -> exact GetCurrentTimestamp() -> GetTemporalReading(),
            // while historical lookup at t0 (before keypress) still resolves the initial baseline,
            // and GetAsyncKeyState's pressed-since bit (bit 0) is NOT consumed by GameInput snapshots.
            let _ = crate::user32::input::GetAsyncKeyState(b'A' as i32);
            let _ = crate::user32::input::GetAsyncKeyState(keymap::VK_LBUTTON as i32);
            crate::user32::input::native_key(NativeKey { mac: 0x00, down: true, repeat: false, chars: vec![b'a' as u16] });
            let tk = gi_get_current_timestamp(gi);
            assert_eq!(gi_get_temporal_reading(gi, tk, kdev, &mut out), S_OK);
            let mut keys = [GameInputKeyState::default(); 8];
            let n_keys = rd_get_key_state(out, keys.len() as u32, keys.as_mut_ptr()) as usize;
            assert!(
                keys[..n_keys].iter().any(|k| k.virtual_key == b'A'),
                "keyboard GetTemporalReading(GetCurrentTimestamp()) must include pressed 'A'"
            );
            assert_eq!(rd_release(out), 0);
            assert_eq!(gi_get_temporal_reading(gi, t0, kdev, &mut out), S_OK);
            let n_base_keys = rd_get_key_state(out, keys.len() as u32, keys.as_mut_ptr()) as usize;
            assert!(
                !keys[..n_base_keys].iter().any(|k| k.virtual_key == b'A'),
                "keyboard baseline at t0 must reflect pre-press state"
            );
            assert_eq!(rd_release(out), 0);
            let async_a = crate::user32::input::GetAsyncKeyState(b'A' as i32);
            assert!(async_a < 0 && (async_a & 1) != 0, "GameInput snapshot must not consume GetAsyncKeyState pressed-since bit for keyboard");
            crate::user32::input::native_key(NativeKey { mac: 0x00, down: false, repeat: false, chars: Vec::new() });

            crate::user32::input::native_mouse(NativeMouse::Button {
                hwnd: 0,
                client: None,
                screen: POINT { x: 100, y: 100 },
                button: MouseButton::Left,
                down: true,
                clicks: 1,
            });
            let async_lbtn = crate::user32::input::GetAsyncKeyState(keymap::VK_LBUTTON as i32);
            assert!(async_lbtn < 0 && (async_lbtn & 1) != 0, "GameInput snapshot must not consume GetAsyncKeyState pressed-since bit for mouse");
            crate::user32::input::native_mouse(NativeMouse::Button {
                hwnd: 0,
                client: None,
                screen: POINT { x: 100, y: 100 },
                button: MouseButton::Left,
                down: false,
                clicks: 1,
            });

            assert_eq!(gi_release(gi), 0);
        }

        reset_test_state();
    }

    #[test]
    fn temporal_reading_records_merged_slot0_disconnect_transitions() {
        let _g = TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        reset_test_state();

        unsafe {
            let mut gi_raw = 0usize;
            assert_eq!(GameInputCreate(&mut gi_raw), S_OK);
            let gi = gi_raw as *mut GameInputObj;
            let dev0 = device_ptr(DeviceKind::Gamepad(0));
            let mut out: *mut ReadingObj = std::ptr::null_mut();
            let mut gs = GameInputGamepadState::default();

            // 1. Physical slot 0 holds A while virtual touch pad is connected and idle.
            xinput1_4::set_virtual_gamepad(true, xinput1_4::Gamepad::default());
            xinput1_4::backend_connect(
                0,
                xinput1_4::DeviceInfo { name: "PhysPad".into(), wireless: false, rumble: false, battery: None },
            );
            xinput1_4::backend_update(0, xinput1_4::Gamepad { buttons: xinput1_4::btn::A, ..Default::default() });
            let t_phys_a = gi_get_current_timestamp(gi);
            assert_eq!(gi_get_temporal_reading(gi, t_phys_a, dev0, &mut out), S_OK);
            assert_eq!(rd_get_gamepad_state(out, &mut gs), 1);
            assert_eq!(gs.buttons, GAME_INPUT_GAMEPAD_A);
            assert_eq!(rd_release(out), 0);

            // Disconnecting the physical controller while the virtual pad remains connected & idle
            // must immediately record the merged release transition at event time.
            std::thread::sleep(std::time::Duration::from_millis(2));
            xinput1_4::backend_disconnect(0);
            let t_after_phys_disc = gi_get_current_timestamp(gi);
            assert!(t_after_phys_disc > t_phys_a);
            assert_eq!(gi_get_temporal_reading(gi, t_after_phys_disc, dev0, &mut out), S_OK);
            assert_eq!(rd_get_gamepad_state(out, &mut gs), 1);
            assert_eq!(gs.buttons, 0, "disconnecting physical pad must record release of A");
            assert_eq!(rd_release(out), 0);
            // Historical timestamp still sees A held.
            assert_eq!(gi_get_temporal_reading(gi, t_phys_a, dev0, &mut out), S_OK);
            assert_eq!(rd_get_gamepad_state(out, &mut gs), 1);
            assert_eq!(gs.buttons, GAME_INPUT_GAMEPAD_A);
            assert_eq!(rd_release(out), 0);

            // 2. Physical slot 0 holds A while virtual pad holds B; disconnecting virtual pad
            // must immediately transition merged state from A|B to A.
            std::thread::sleep(std::time::Duration::from_millis(2));
            xinput1_4::backend_connect(
                0,
                xinput1_4::DeviceInfo { name: "PhysPad".into(), wireless: false, rumble: false, battery: None },
            );
            xinput1_4::backend_update(0, xinput1_4::Gamepad { buttons: xinput1_4::btn::A, ..Default::default() });
            xinput1_4::set_virtual_gamepad(true, xinput1_4::Gamepad { buttons: xinput1_4::btn::B, ..Default::default() });
            let t_merged_ab = gi_get_current_timestamp(gi);
            assert_eq!(gi_get_temporal_reading(gi, t_merged_ab, dev0, &mut out), S_OK);
            assert_eq!(rd_get_gamepad_state(out, &mut gs), 1);
            assert_eq!(gs.buttons, GAME_INPUT_GAMEPAD_A | GAME_INPUT_GAMEPAD_B);
            assert_eq!(rd_release(out), 0);

            std::thread::sleep(std::time::Duration::from_millis(2));
            xinput1_4::set_virtual_gamepad(false, xinput1_4::Gamepad::default());
            let t_after_virt_disc = gi_get_current_timestamp(gi);
            assert!(t_after_virt_disc > t_merged_ab);
            assert_eq!(gi_get_temporal_reading(gi, t_after_virt_disc, dev0, &mut out), S_OK);
            assert_eq!(rd_get_gamepad_state(out, &mut gs), 1);
            assert_eq!(gs.buttons, GAME_INPUT_GAMEPAD_A, "disconnecting virtual pad must record release of B");
            assert_eq!(rd_release(out), 0);

            // Historical timestamp still sees A|B held.
            assert_eq!(gi_get_temporal_reading(gi, t_merged_ab, dev0, &mut out), S_OK);
            assert_eq!(rd_get_gamepad_state(out, &mut gs), 1);
            assert_eq!(gs.buttons, GAME_INPUT_GAMEPAD_A | GAME_INPUT_GAMEPAD_B);
            assert_eq!(rd_release(out), 0);

            // 3. Physical idle + virtual holding B -> disconnecting virtual pad transitions to idle.
            std::thread::sleep(std::time::Duration::from_millis(2));
            xinput1_4::backend_update(0, xinput1_4::Gamepad::default());
            xinput1_4::set_virtual_gamepad(true, xinput1_4::Gamepad { buttons: xinput1_4::btn::B, ..Default::default() });
            let t_virt_b = gi_get_current_timestamp(gi);
            assert_eq!(gi_get_temporal_reading(gi, t_virt_b, dev0, &mut out), S_OK);
            assert_eq!(rd_get_gamepad_state(out, &mut gs), 1);
            assert_eq!(gs.buttons, GAME_INPUT_GAMEPAD_B);
            assert_eq!(rd_release(out), 0);

            std::thread::sleep(std::time::Duration::from_millis(2));
            xinput1_4::set_virtual_gamepad(false, xinput1_4::Gamepad::default());
            let t_after_virt_idle = gi_get_current_timestamp(gi);
            assert_eq!(gi_get_temporal_reading(gi, t_after_virt_idle, dev0, &mut out), S_OK);
            assert_eq!(rd_get_gamepad_state(out, &mut gs), 1);
            assert_eq!(gs.buttons, 0, "disconnecting virtual pad over idle physical pad must record idle");
            assert_eq!(rd_release(out), 0);

            assert_eq!(gi_release(gi), 0);
        }

        reset_test_state();
    }
}
