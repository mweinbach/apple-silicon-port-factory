//! dinput8.dll / `CLSID_DirectInput8`: DirectInput 8 COM runtime exposing
//! connected `GameController` pads (`xinput1_4` slots 0..4).
//!
//! * `IDirectInput8W` / `IDirectInput8A`: enumerates connected gamepads via
//!   `EnumDevices` (matching `DI8DEVCLASS_ALL`, `DI8DEVCLASS_GAMECTRL`,
//!   `DI8DEVTYPE_GAMEPAD`, `DI8DEVTYPE_JOYSTICK`, `DI8DEVTYPE_1STPERSON`) and
//!   creates live `IDirectInputDevice8W` / `IDirectInputDevice8A` objects via
//!   `CreateDevice`.
//! * Force feedback (`DIEDFL_FORCEFEEDBACK`): XInput / `GameController` pads
//!   do not expose DirectInput force feedback on Windows either, so
//!   `EnumDevices(..., DIEDFL_FORCEFEEDBACK)` reports no devices. Wwise's
//!   `AkDirectInputRumbleSink` therefore falls back cleanly to XInput rumble.
//! * `IDirectInputDevice8W` / `IDirectInputDevice8A`: supports `SetDataFormat`
//!   (`DIJOYSTATE`, `DIJOYSTATE2`, and custom `DIDATAFORMAT`), `SetCooperativeLevel`,
//!   `Acquire`, `Unacquire`, `Poll`, `GetDeviceState`, `GetDeviceData`,
//!   `GetProperty` / `SetProperty` (`DIPROP_RANGE`, `DIPROP_DEADZONE`,
//!   `DIPROP_SATURATION`, `DIPROP_AXISMODE`, `DIPROP_BUFFERSIZE`,
//!   `DIPROP_GRANULARITY`, `DIPROP_VIDPID`, `DIPROP_JOYSTICKID`,
//!   `DIPROP_INSTANCENAME`, `DIPROP_PRODUCTNAME`), `GetCapabilities`,
//!   `GetDeviceInfo`, `GetObjectInfo`, and `EnumObjects`.

use crate::misc::xinput1_4::{self, btn, Gamepad, XUSER_MAX_COUNT};
use std::collections::{HashMap, VecDeque};
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

type HRESULT = i32;
const S_OK: HRESULT = 0;
const DI_OK: HRESULT = 0;
const DI_NOTATTACHED: HRESULT = 1; // S_FALSE
const DI_NOEFFECT: HRESULT = 1; // S_FALSE
const DI_PROPNOEFFECT: HRESULT = 1; // S_FALSE
const DI_BUFFEROVERFLOW: HRESULT = 1; // S_FALSE
const E_POINTER: HRESULT = 0x8000_4003u32 as i32;
const E_NOINTERFACE: HRESULT = 0x8000_4002u32 as i32;
const CLASS_E_NOAGGREGATION: HRESULT = 0x8004_0110u32 as i32;
const DIERR_DEVICENOTREG: HRESULT = 0x8004_0154u32 as i32; // REGDB_E_CLASSNOTREG
const DIERR_UNSUPPORTED: HRESULT = 0x8000_4001u32 as i32; // E_NOTIMPL
const DIERR_INVALIDPARAM: HRESULT = 0x8007_0057u32 as i32; // E_INVALIDARG
const DIERR_OBJECTNOTFOUND: HRESULT = 0x8007_0002u32 as i32; // ERROR_FILE_NOT_FOUND
const DIERR_NOTACQUIRED: HRESULT = 0x8007_000Cu32 as i32; // ERROR_INVALID_ACCESS
const DIERR_INPUTLOST: HRESULT = 0x8007_001Eu32 as i32; // ERROR_READ_FAULT
const DIERR_BETADIRECTINPUTVERSION: HRESULT = 0x8007_0481u32 as i32;
const DIERR_OLDDIRECTINPUTVERSION: HRESULT = 0x8007_047Eu32 as i32;

const DIENUM_STOP: i32 = 0;

// Device classes & types
const DI8DEVCLASS_ALL: u32 = 0;
const DI8DEVCLASS_GAMECTRL: u32 = 4;
const DI8DEVTYPE_JOYSTICK: u32 = 0x14;
const DI8DEVTYPE_GAMEPAD: u32 = 0x15;
const DI8DEVTYPE_1STPERSON: u32 = 0x18;
const DI8DEVTYPEGAMEPAD_STANDARD: u32 = 2;
const GAMEPAD_DEVTYPE: u32 = DI8DEVTYPE_GAMEPAD | (DI8DEVTYPEGAMEPAD_STANDARD << 8);

// Enumeration flags
const DIEDFL_FORCEFEEDBACK: u32 = 0x0000_0100;

// Device capability flags
const DIDC_ATTACHED: u32 = 0x0000_0001;

// Cooperative level flags
const DISCL_EXCLUSIVE: u32 = 0x0000_0001;
const DISCL_NONEXCLUSIVE: u32 = 0x0000_0002;
const DISCL_FOREGROUND: u32 = 0x0000_0004;
const DISCL_BACKGROUND: u32 = 0x0000_0008;

// Object type flags (DIDFT_*)
const DIDFT_ALL: u32 = 0x0000_0000;
const DIDFT_ABSAXIS: u32 = 0x0000_0002;
const DIDFT_AXIS: u32 = 0x0000_0003;
const DIDFT_PSHBUTTON: u32 = 0x0000_0004;
const DIDFT_BUTTON: u32 = 0x0000_000C;
const DIDFT_POV: u32 = 0x0000_0010;
const DIDFT_ANYINSTANCE: u32 = 0x00FF_FF00;
const DIDOI_ASPECTPOSITION: u32 = 0x0000_0100;

// Property how flags (DIPH_*)
const DIPH_DEVICE: u32 = 0;
const DIPH_BYOFFSET: u32 = 1;
const DIPH_BYID: u32 = 2;

// Property IDs (MAKEDIPROP(n), dinput.h)
const DIPROP_BUFFERSIZE: usize = 1;
const DIPROP_AXISMODE: usize = 2;
const DIPROP_GRANULARITY: usize = 3;
const DIPROP_RANGE: usize = 4;
const DIPROP_DEADZONE: usize = 5;
const DIPROP_SATURATION: usize = 6;
const DIPROP_FFGAIN: usize = 7;
const DIPROP_FFLOAD: usize = 8;
const DIPROP_AUTOCENTER: usize = 9;
const DIPROP_CALIBRATIONMODE: usize = 10;
const DIPROP_CALIBRATION: usize = 11;
const DIPROP_GUIDANDPATH: usize = 12;
const DIPROP_INSTANCENAME: usize = 13;
const DIPROP_PRODUCTNAME: usize = 14;
const DIPROP_JOYSTICKID: usize = 15;
const DIPROP_GETPORTDISPLAYNAME: usize = 16;
const DIPROP_PHYSICALRANGE: usize = 18;
const DIPROP_LOGICALRANGE: usize = 19;
const DIPROP_KEYNAME: usize = 20;
const DIPROP_APPDATA: usize = 22;
const DIPROP_VIDPID: usize = 24;
const DIPROP_USERNAME: usize = 25;
const DIPROP_TYPENAME: usize = 26;

const DIPROPAXISMODE_ABS: u32 = 0;
const DIPROPAUTOCENTER_OFF: u32 = 0;
const DIPROPAUTOCENTER_ON: u32 = 1;
const DIPROPCALIBRATIONMODE_COOKED: u32 = 0;
const DIPROPCALIBRATIONMODE_RAW: u32 = 1;
const DIGDD_PEEK: u32 = 0x0000_0001;

// Standard Xbox-compatible VID/PID (0x045E Microsoft, 0x028E Xbox 360 Controller)
const GAMEPAD_VIDPID: u32 = (0x028E << 16) | 0x045E;

type Guid = [u8; 16];

const fn guid(d1: u32, d2: u16, d3: u16, d4: [u8; 8]) -> Guid {
    let a = d1.to_le_bytes();
    let b = d2.to_le_bytes();
    let c = d3.to_le_bytes();
    [a[0], a[1], a[2], a[3], b[0], b[1], c[0], c[1], d4[0], d4[1], d4[2], d4[3], d4[4], d4[5], d4[6], d4[7]]
}

pub const CLSID_DIRECTINPUT8: Guid = guid(0x25E609E4, 0xB259, 0x11CF, [0xBF, 0xC7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00]);
const IID_IDIRECTINPUT8W: Guid = guid(0xBF798031, 0x483A, 0x4DA2, [0xAA, 0x99, 0x5D, 0x64, 0xED, 0x36, 0x97, 0x00]);
const IID_IDIRECTINPUT8A: Guid = guid(0xBF798030, 0x483A, 0x4DA2, [0xAA, 0x99, 0x5D, 0x64, 0xED, 0x36, 0x97, 0x00]);
const IID_IDIRECTINPUTDEVICE8W: Guid = guid(0x54D41081, 0xDC15, 0x4833, [0xA4, 0x1B, 0x74, 0x8F, 0x73, 0xA3, 0x81, 0x79]);
const IID_IDIRECTINPUTDEVICE8A: Guid = guid(0x54D41080, 0xDC15, 0x4833, [0xA4, 0x1B, 0x74, 0x8F, 0x73, 0xA3, 0x81, 0x79]);
const IID_IUNKNOWN: Guid = guid(0x00000000, 0x0000, 0x0000, [0xC0, 0, 0, 0, 0, 0, 0, 0x46]);

// Standard DirectInput object type GUIDs
const GUID_XAXIS: Guid = guid(0xA36D02E0, 0xC9F3, 0x11CF, [0xBF, 0xC7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00]);
const GUID_YAXIS: Guid = guid(0xA36D02E1, 0xC9F3, 0x11CF, [0xBF, 0xC7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00]);
const GUID_ZAXIS: Guid = guid(0xA36D02E2, 0xC9F3, 0x11CF, [0xBF, 0xC7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00]);
const GUID_RXAXIS: Guid = guid(0xA36D02F4, 0xC9F3, 0x11CF, [0xBF, 0xC7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00]);
const GUID_RYAXIS: Guid = guid(0xA36D02F5, 0xC9F3, 0x11CF, [0xBF, 0xC7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00]);
const GUID_RZAXIS: Guid = guid(0xA36D02E3, 0xC9F3, 0x11CF, [0xBF, 0xC7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00]);
const GUID_SLIDER: Guid = guid(0xA36D02E4, 0xC9F3, 0x11CF, [0xBF, 0xC7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00]);
const GUID_BUTTON: Guid = guid(0xA36D02F0, 0xC9F3, 0x11CF, [0xBF, 0xC7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00]);
const GUID_POV: Guid = guid(0xA36D02F2, 0xC9F3, 0x11CF, [0xBF, 0xC7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00]);

const GAMEPAD_PRODUCT_GUID: Guid = guid(GAMEPAD_VIDPID, 0x0000, 0x0000, [0x00, 0x00, b'P', b'I', b'D', b'V', b'I', b'D']);

const fn slot_guid(slot: usize) -> Guid {
    guid(0xD108_0000, 0x0000, 0x0000, [0x00, 0x00, b'P', b'A', b'D', 0x00, 0x00, slot as u8])
}

fn slot_from_guid(g: &Guid) -> Option<usize> {
    (0..XUSER_MAX_COUNT).find(|&s| *g == slot_guid(s))
}

fn slot_instance_name(slot: usize) -> String {
    format!("Controller (Gamepad {})", slot + 1)
}

const PRODUCT_NAME: &str = "GameController Gamepad";

static GAMEPAD_STARTED: AtomicBool = AtomicBool::new(false);

fn ensure_gamepad_started() {
    if !GAMEPAD_STARTED.swap(true, Ordering::AcqRel) {
        crate::macos::gamepad::start();
    }
}

// ---------------------------------------------------------------------------
// DirectInput 8 Win64 ABI structs
// ---------------------------------------------------------------------------

const MAX_PATH: usize = 260;

#[repr(C)]
#[derive(Clone, Copy)]
struct DIDEVICEINSTANCEA {
    dw_size: u32,
    guid_instance: Guid,
    guid_product: Guid,
    dw_dev_type: u32,
    tsz_instance_name: [u8; MAX_PATH],
    tsz_product_name: [u8; MAX_PATH],
    guid_ff_driver: Guid,
    w_usage_page: u16,
    w_usage: u16,
}
const _: () = assert!(std::mem::size_of::<DIDEVICEINSTANCEA>() == 580);

#[repr(C)]
#[derive(Clone, Copy)]
struct DIDEVICEINSTANCEW {
    dw_size: u32,
    guid_instance: Guid,
    guid_product: Guid,
    dw_dev_type: u32,
    tsz_instance_name: [u16; MAX_PATH],
    tsz_product_name: [u16; MAX_PATH],
    guid_ff_driver: Guid,
    w_usage_page: u16,
    w_usage: u16,
}
const _: () = assert!(std::mem::size_of::<DIDEVICEINSTANCEW>() == 1100);

#[repr(C)]
#[derive(Clone, Copy)]
struct DIDEVICEOBJECTINSTANCEA {
    dw_size: u32,
    guid_type: Guid,
    dw_ofs: u32,
    dw_type: u32,
    dw_flags: u32,
    tsz_name: [u8; MAX_PATH],
    dw_ff_max_force: u32,
    dw_ff_force_resolution: u32,
    w_collection_number: u16,
    w_designator_index: u16,
    w_usage_page: u16,
    w_usage: u16,
    dw_dimension: u32,
    w_exponent: u16,
    w_report_id: u16,
}
const _: () = assert!(std::mem::size_of::<DIDEVICEOBJECTINSTANCEA>() == 316);

#[repr(C)]
#[derive(Clone, Copy)]
struct DIDEVICEOBJECTINSTANCEW {
    dw_size: u32,
    guid_type: Guid,
    dw_ofs: u32,
    dw_type: u32,
    dw_flags: u32,
    tsz_name: [u16; MAX_PATH],
    dw_ff_max_force: u32,
    dw_ff_force_resolution: u32,
    w_collection_number: u16,
    w_designator_index: u16,
    w_usage_page: u16,
    w_usage: u16,
    dw_dimension: u32,
    w_exponent: u16,
    w_report_id: u16,
}
const _: () = assert!(std::mem::size_of::<DIDEVICEOBJECTINSTANCEW>() == 576);

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct DIDEVCAPS {
    dw_size: u32,
    dw_flags: u32,
    dw_dev_type: u32,
    dw_axes: u32,
    dw_buttons: u32,
    dw_povs: u32,
    dw_ff_sample_period: u32,
    dw_ff_min_time_resolution: u32,
    dw_firmware_revision: u32,
    dw_hardware_revision: u32,
    dw_ff_driver_version: u32,
}
const _: () = assert!(std::mem::size_of::<DIDEVCAPS>() == 44);

#[repr(C)]
#[derive(Clone, Copy)]
struct DIOBJECTDATAFORMAT {
    pguid: *const Guid,
    dw_ofs: u32,
    dw_type: u32,
    dw_flags: u32,
}
const _: () = assert!(std::mem::size_of::<DIOBJECTDATAFORMAT>() == 24);

#[repr(C)]
#[derive(Clone, Copy)]
struct DIDATAFORMAT {
    dw_size: u32,
    dw_obj_size: u32,
    dw_flags: u32,
    dw_data_size: u32,
    dw_num_objs: u32,
    rgodf: *const DIOBJECTDATAFORMAT,
}
const _: () = assert!(std::mem::size_of::<DIDATAFORMAT>() == 32);

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct DIPROPHEADER {
    dw_size: u32,
    dw_header_size: u32,
    dw_obj: u32,
    dw_how: u32,
}
const _: () = assert!(std::mem::size_of::<DIPROPHEADER>() == 16);

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct DIPROPDWORD {
    diph: DIPROPHEADER,
    dw_data: u32,
}
const _: () = assert!(std::mem::size_of::<DIPROPDWORD>() == 20);

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct DIPROPRANGE {
    diph: DIPROPHEADER,
    l_min: i32,
    l_max: i32,
}
const _: () = assert!(std::mem::size_of::<DIPROPRANGE>() == 24);

#[repr(C)]
#[derive(Clone, Copy)]
struct DIPROPSTRING {
    diph: DIPROPHEADER,
    wsz: [u16; MAX_PATH],
}
const _: () = assert!(std::mem::size_of::<DIPROPSTRING>() == 536);

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct DIPROPPOINTER {
    diph: DIPROPHEADER,
    u_data: usize,
}
const _: () = assert!(std::mem::size_of::<DIPROPPOINTER>() == 24);

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct DIPROPCAL {
    diph: DIPROPHEADER,
    l_min: i32,
    l_center: i32,
    l_max: i32,
}
const _: () = assert!(std::mem::size_of::<DIPROPCAL>() == 28);

#[repr(C)]
#[derive(Clone, Copy)]
struct DIPROPGUIDANDPATH {
    diph: DIPROPHEADER,
    guid_class: Guid,
    wsz_path: [u16; MAX_PATH],
}
const _: () = assert!(std::mem::size_of::<DIPROPGUIDANDPATH>() == 552);

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct DIDEVICEOBJECTDATA {
    dw_ofs: u32,
    dw_data: u32,
    dw_time_stamp: u32,
    dw_sequence: u32,
    u_app_data: usize,
}
const _: () = assert!(std::mem::size_of::<DIDEVICEOBJECTDATA>() == 24);

#[repr(C)]
#[derive(Clone, Copy)]
struct DIJOYSTATE {
    l_x: i32,
    l_y: i32,
    l_z: i32,
    l_rx: i32,
    l_ry: i32,
    l_rz: i32,
    rgl_slider: [i32; 2],
    rgdw_pov: [u32; 4],
    rgb_buttons: [u8; 32],
}
const _: () = assert!(std::mem::size_of::<DIJOYSTATE>() == 80);

#[repr(C)]
#[derive(Clone, Copy)]
struct DIJOYSTATE2 {
    l_x: i32,
    l_y: i32,
    l_z: i32,
    l_rx: i32,
    l_ry: i32,
    l_rz: i32,
    rgl_slider: [i32; 2],
    rgdw_pov: [u32; 4],
    rgb_buttons: [u8; 128],
    l_vx: i32,
    l_vy: i32,
    l_vz: i32,
    l_vrx: i32,
    l_vry: i32,
    l_vrz: i32,
    rgl_vslider: [i32; 2],
    l_ax: i32,
    l_ay: i32,
    l_az: i32,
    l_arx: i32,
    l_ary: i32,
    l_arz: i32,
    rgl_aslider: [i32; 2],
    l_fx: i32,
    l_fy: i32,
    l_fz: i32,
    l_frx: i32,
    l_fry: i32,
    l_frz: i32,
    rgl_fslider: [i32; 2],
}
const _: () = assert!(std::mem::size_of::<DIJOYSTATE2>() == 272);

// ---------------------------------------------------------------------------
// Gamepad object descriptors & state mapping
// ---------------------------------------------------------------------------

const NUM_AXES: usize = 8; // X, Y, Z, Rx, Ry, Rz, Slider0, Slider1
const NUM_BUTTONS: usize = 12;
const DEFAULT_AXIS_MIN: i32 = 0;
const DEFAULT_AXIS_MAX: i32 = 65535;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AxisId {
    X = 0,
    Y = 1,
    Z = 2,
    Rx = 3,
    Ry = 4,
    Rz = 5,
    Slider0 = 6,
    Slider1 = 7,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ObjKind {
    Axis(AxisId),
    Pov(u8),
    Button(u8),
}

#[derive(Clone, Copy, Debug)]
struct MappedObj {
    kind: ObjKind,
    ofs: u32,
}

#[derive(Clone, Copy)]
struct NativeObjDesc {
    kind: ObjKind,
    guid: Guid,
    dw_type: u32,
    default_ofs: u32,
    usage_page: u16,
    usage: u16,
    name: &'static str,
}

const fn make_didft_instance(inst: u32) -> u32 {
    (inst & 0xFFFF) << 8
}

fn native_objects() -> Vec<NativeObjDesc> {
    let mut v = Vec::with_capacity(20);
    // Standard 7 active axes (lX, lY, lZ, lRx, lRy, Slider 0, Slider 1).
    let axes: &[(AxisId, Guid, u32, u16, &'static str)] = &[
        (AxisId::X, GUID_XAXIS, 0, 0x30, "X Axis"),
        (AxisId::Y, GUID_YAXIS, 4, 0x31, "Y Axis"),
        (AxisId::Z, GUID_ZAXIS, 8, 0x32, "Z Axis"),
        (AxisId::Rx, GUID_RXAXIS, 12, 0x33, "X Rotation"),
        (AxisId::Ry, GUID_RYAXIS, 16, 0x34, "Y Rotation"),
        (AxisId::Slider0, GUID_SLIDER, 24, 0x36, "Left Trigger"),
        (AxisId::Slider1, GUID_SLIDER, 28, 0x36, "Right Trigger"),
    ];
    for (idx, &(aid, g, ofs, usage, name)) in axes.iter().enumerate() {
        v.push(NativeObjDesc {
            kind: ObjKind::Axis(aid),
            guid: g,
            dw_type: DIDFT_ABSAXIS | make_didft_instance(idx as u32),
            default_ofs: ofs,
            usage_page: 0x01,
            usage,
            name,
        });
    }
    // 1 POV hat (D-pad).
    v.push(NativeObjDesc {
        kind: ObjKind::Pov(0),
        guid: GUID_POV,
        dw_type: DIDFT_POV | make_didft_instance(0),
        default_ofs: 32,
        usage_page: 0x01,
        usage: 0x39,
        name: "Hat Switch",
    });
    // 12 buttons.
    const BTN_NAMES: [&str; NUM_BUTTONS] = [
        "Button 0",
        "Button 1",
        "Button 2",
        "Button 3",
        "Button 4",
        "Button 5",
        "Button 6",
        "Button 7",
        "Button 8",
        "Button 9",
        "Button 10",
        "Button 11",
    ];
    for (i, &name) in BTN_NAMES.iter().enumerate() {
        v.push(NativeObjDesc {
            kind: ObjKind::Button(i as u8),
            guid: GUID_BUTTON,
            dw_type: DIDFT_PSHBUTTON | make_didft_instance(i as u32),
            default_ofs: 48 + i as u32,
            usage_page: 0x09,
            usage: (i + 1) as u16,
            name,
        });
    }
    v
}

fn fill_ascii_fixed(dst: &mut [u8], s: &str) {
    dst.fill(0);
    let b = s.as_bytes();
    let n = b.len().min(dst.len().saturating_sub(1));
    dst[..n].copy_from_slice(&b[..n]);
}

fn fill_wide_fixed(dst: &mut [u16], s: &str) {
    dst.fill(0);
    for (i, u) in s.encode_utf16().take(dst.len().saturating_sub(1)).enumerate() {
        dst[i] = u;
    }
}

fn make_device_instance_a(slot: usize) -> DIDEVICEINSTANCEA {
    let mut inst = DIDEVICEINSTANCEA {
        dw_size: std::mem::size_of::<DIDEVICEINSTANCEA>() as u32,
        guid_instance: slot_guid(slot),
        guid_product: GAMEPAD_PRODUCT_GUID,
        dw_dev_type: GAMEPAD_DEVTYPE,
        tsz_instance_name: [0; MAX_PATH],
        tsz_product_name: [0; MAX_PATH],
        guid_ff_driver: [0; 16],
        w_usage_page: 0x01,
        w_usage: 0x05,
    };
    fill_ascii_fixed(&mut inst.tsz_instance_name, &slot_instance_name(slot));
    fill_ascii_fixed(&mut inst.tsz_product_name, PRODUCT_NAME);
    inst
}

fn make_device_instance_w(slot: usize) -> DIDEVICEINSTANCEW {
    let mut inst = DIDEVICEINSTANCEW {
        dw_size: std::mem::size_of::<DIDEVICEINSTANCEW>() as u32,
        guid_instance: slot_guid(slot),
        guid_product: GAMEPAD_PRODUCT_GUID,
        dw_dev_type: GAMEPAD_DEVTYPE,
        tsz_instance_name: [0; MAX_PATH],
        tsz_product_name: [0; MAX_PATH],
        guid_ff_driver: [0; 16],
        w_usage_page: 0x01,
        w_usage: 0x05,
    };
    fill_wide_fixed(&mut inst.tsz_instance_name, &slot_instance_name(slot));
    fill_wide_fixed(&mut inst.tsz_product_name, PRODUCT_NAME);
    inst
}

fn make_obj_instance_a(desc: &NativeObjDesc, ofs: u32) -> DIDEVICEOBJECTINSTANCEA {
    let mut out = DIDEVICEOBJECTINSTANCEA {
        dw_size: std::mem::size_of::<DIDEVICEOBJECTINSTANCEA>() as u32,
        guid_type: desc.guid,
        dw_ofs: ofs,
        dw_type: desc.dw_type,
        dw_flags: if matches!(desc.kind, ObjKind::Axis(_)) { DIDOI_ASPECTPOSITION } else { 0 },
        tsz_name: [0; MAX_PATH],
        dw_ff_max_force: 0,
        dw_ff_force_resolution: 0,
        w_collection_number: 0,
        w_designator_index: 0,
        w_usage_page: desc.usage_page,
        w_usage: desc.usage,
        dw_dimension: 0,
        w_exponent: 0,
        w_report_id: 0,
    };
    fill_ascii_fixed(&mut out.tsz_name, desc.name);
    out
}

fn make_obj_instance_w(desc: &NativeObjDesc, ofs: u32) -> DIDEVICEOBJECTINSTANCEW {
    let mut out = DIDEVICEOBJECTINSTANCEW {
        dw_size: std::mem::size_of::<DIDEVICEOBJECTINSTANCEW>() as u32,
        guid_type: desc.guid,
        dw_ofs: ofs,
        dw_type: desc.dw_type,
        dw_flags: if matches!(desc.kind, ObjKind::Axis(_)) { DIDOI_ASPECTPOSITION } else { 0 },
        tsz_name: [0; MAX_PATH],
        dw_ff_max_force: 0,
        dw_ff_force_resolution: 0,
        w_collection_number: 0,
        w_designator_index: 0,
        w_usage_page: desc.usage_page,
        w_usage: desc.usage,
        dw_dimension: 0,
        w_exponent: 0,
        w_report_id: 0,
    };
    fill_wide_fixed(&mut out.tsz_name, desc.name);
    out
}

fn scale_signed_norm(mut norm: f64, lmin: i32, lmax: i32, deadzone: u32, saturation: u32) -> i32 {
    if lmin >= lmax {
        return lmin;
    }
    let dz = (deadzone.min(10000) as f64) / 10000.0;
    let sat = (saturation.clamp(1, 10000) as f64) / 10000.0;
    let mag = norm.abs();
    if mag <= dz {
        norm = 0.0;
    } else if mag >= sat {
        norm = norm.signum();
    } else if dz > 0.0 || sat < 1.0 {
        norm = norm.signum() * ((mag - dz) / (sat - dz).max(1e-6));
    }
    if norm == 0.0 {
        return ((lmin as i64 + lmax as i64) / 2) as i32;
    }
    let mid = (lmin as i64 + lmax as i64) as f64 / 2.0;
    let scaled = if norm < 0.0 {
        mid + norm * (mid - lmin as f64)
    } else {
        mid + norm * (lmax as f64 - mid)
    };
    scaled.round().clamp(lmin as f64, lmax as f64) as i32
}

fn scale_i16_axis(raw: i16, invert: bool, lmin: i32, lmax: i32, deadzone: u32, saturation: u32) -> i32 {
    let mut norm = if raw < 0 { raw as f64 / 32768.0 } else { raw as f64 / 32767.0 };
    if invert {
        norm = -norm;
    }
    scale_signed_norm(norm, lmin, lmax, deadzone, saturation)
}

fn scale_unit_trigger(mut t: f64, lmin: i32, lmax: i32, deadzone: u32, saturation: u32) -> i32 {
    if lmin >= lmax {
        return lmin;
    }
    let dz = (deadzone.min(10000) as f64) / 10000.0;
    let sat = (saturation.clamp(1, 10000) as f64) / 10000.0;
    if t <= dz {
        t = 0.0;
    } else if t >= sat {
        t = 1.0;
    } else if dz > 0.0 || sat < 1.0 {
        t = (t - dz) / (sat - dz).max(1e-6);
    }
    let span = (lmax as i64 - lmin as i64) as f64;
    (lmin as f64 + t * span).round().clamp(lmin as f64, lmax as f64) as i32
}

fn scale_u8_trigger(raw: u8, lmin: i32, lmax: i32, deadzone: u32, saturation: u32) -> i32 {
    scale_unit_trigger((raw as f64) / 255.0, lmin, lmax, deadzone, saturation)
}

fn dpad_pov(buttons: u16) -> u32 {
    let up = buttons & btn::DPAD_UP != 0;
    let down = buttons & btn::DPAD_DOWN != 0;
    let left = buttons & btn::DPAD_LEFT != 0;
    let right = buttons & btn::DPAD_RIGHT != 0;
    let u = up && !down;
    let d = down && !up;
    let l = left && !right;
    let r = right && !left;
    match (u, d, l, r) {
        (true, false, false, false) => 0,
        (true, false, false, true) => 4500,
        (false, false, false, true) => 9000,
        (false, true, false, true) => 13500,
        (false, true, false, false) => 18000,
        (false, true, true, false) => 22500,
        (false, false, true, false) => 27000,
        (true, false, true, false) => 31500,
        _ => u32::MAX, // centered (-1)
    }
}

fn button_pressed(pad: &Gamepad, idx: u8) -> bool {
    let bit = match idx {
        0 => btn::A,
        1 => btn::B,
        2 => btn::X,
        3 => btn::Y,
        4 => btn::LEFT_SHOULDER,
        5 => btn::RIGHT_SHOULDER,
        6 => btn::BACK,
        7 => btn::START,
        8 => btn::LEFT_THUMB,
        9 => btn::RIGHT_THUMB,
        10 => btn::GUIDE,
        _ => return false,
    };
    pad.buttons & bit != 0
}

const DEFAULT_AXIS_CAL: (i32, i32, i32) = (DEFAULT_AXIS_MIN, 32_767, DEFAULT_AXIS_MAX);

fn axis_value(pad: &Gamepad, aid: AxisId, range: (i32, i32), dz: u32, sat: u32) -> i32 {
    let (lmin, lmax) = range;
    match aid {
        AxisId::X => scale_i16_axis(pad.thumb_lx, false, lmin, lmax, dz, sat),
        // XInput +Y is up; DirectInput lY has up = lMin and down = lMax.
        AxisId::Y => scale_i16_axis(pad.thumb_ly, true, lmin, lmax, dz, sat),
        // Standard Windows Xbox DirectInput Z axis: combined triggers (LT -> +max, RT -> -min, centered when idle).
        AxisId::Z => {
            let norm = (pad.left_trigger as f64 - pad.right_trigger as f64) / 255.0;
            scale_signed_norm(norm, lmin, lmax, dz, sat)
        }
        AxisId::Rx => scale_i16_axis(pad.thumb_rx, false, lmin, lmax, dz, sat),
        AxisId::Ry => scale_i16_axis(pad.thumb_ry, true, lmin, lmax, dz, sat),
        AxisId::Rz => scale_signed_norm(0.0, lmin, lmax, dz, sat),
        AxisId::Slider0 => scale_u8_trigger(pad.left_trigger, lmin, lmax, dz, sat),
        AxisId::Slider1 => scale_u8_trigger(pad.right_trigger, lmin, lmax, dz, sat),
    }
}

/// Uncalibrated physical axis value in `DIPROP_PHYSICALRANGE` (`0..=65535`),
/// bypassing `DIPROP_CALIBRATION`, `DIPROP_RANGE`, `DIPROP_DEADZONE`, and
/// `DIPROP_SATURATION` for `DIPROPCALIBRATIONMODE_RAW`.
fn axis_raw_value(pad: &Gamepad, aid: AxisId) -> i32 {
    axis_value(pad, aid, (DEFAULT_AXIS_MIN, DEFAULT_AXIS_MAX), 0, 10_000)
}

fn axis_value_cooked(pad: &Gamepad, aid: AxisId, cal: (i32, i32, i32), range: (i32, i32), dz: u32, sat: u32) -> i32 {
    if cal == DEFAULT_AXIS_CAL {
        return axis_value(pad, aid, range, dz, sat);
    }
    let (cal_min, cal_center, cal_max) = cal;
    if cal_min >= cal_max {
        return axis_value(pad, aid, range, dz, sat);
    }
    let raw = axis_raw_value(pad, aid);
    let (lmin, lmax) = range;
    match aid {
        AxisId::Slider0 | AxisId::Slider1 => {
            let t = ((raw as i64 - cal_min as i64) as f64 / (cal_max as i64 - cal_min as i64) as f64).clamp(0.0, 1.0);
            scale_unit_trigger(t, lmin, lmax, dz, sat)
        }
        _ => {
            let norm = if cal_min < cal_center && cal_center < cal_max {
                if raw < cal_center {
                    ((raw as i64 - cal_center as i64) as f64 / (cal_center as i64 - cal_min as i64) as f64).clamp(-1.0, 0.0)
                } else {
                    ((raw as i64 - cal_center as i64) as f64 / (cal_max as i64 - cal_center as i64) as f64).clamp(0.0, 1.0)
                }
            } else {
                let u = ((raw as i64 - cal_min as i64) as f64 / (cal_max as i64 - cal_min as i64) as f64).clamp(0.0, 1.0);
                u * 2.0 - 1.0
            };
            scale_signed_norm(norm, lmin, lmax, dz, sat)
        }
    }
}

fn standard_joystate_mappings(num_buttons: u8) -> Vec<MappedObj> {
    let mut m = vec![
        MappedObj { kind: ObjKind::Axis(AxisId::X), ofs: 0 },
        MappedObj { kind: ObjKind::Axis(AxisId::Y), ofs: 4 },
        MappedObj { kind: ObjKind::Axis(AxisId::Z), ofs: 8 },
        MappedObj { kind: ObjKind::Axis(AxisId::Rx), ofs: 12 },
        MappedObj { kind: ObjKind::Axis(AxisId::Ry), ofs: 16 },
        MappedObj { kind: ObjKind::Axis(AxisId::Rz), ofs: 20 },
        MappedObj { kind: ObjKind::Axis(AxisId::Slider0), ofs: 24 },
        MappedObj { kind: ObjKind::Axis(AxisId::Slider1), ofs: 28 },
        MappedObj { kind: ObjKind::Pov(0), ofs: 32 },
        MappedObj { kind: ObjKind::Pov(1), ofs: 36 },
        MappedObj { kind: ObjKind::Pov(2), ofs: 40 },
        MappedObj { kind: ObjKind::Pov(3), ofs: 44 },
    ];
    for b in 0..num_buttons {
        m.push(MappedObj { kind: ObjKind::Button(b), ofs: 48 + b as u32 });
    }
    m
}

unsafe fn parse_data_format(df: &DIDATAFORMAT) -> Result<Vec<MappedObj>, HRESULT> {
    if df.rgodf.is_null() || df.dw_num_objs == 0 {
        return match df.dw_data_size {
            80 => Ok(standard_joystate_mappings(32)),
            272 => Ok(standard_joystate_mappings(128)),
            _ => Err(DIERR_INVALIDPARAM),
        };
    }
    let mut out = Vec::with_capacity(df.dw_num_objs as usize);
    let mut next_slider = 0u8;
    let mut next_pov = 0u8;
    let mut next_btn = 0u8;
    let mut next_any_axis = 0u8;
    for i in 0..df.dw_num_objs as usize {
        let obj = df.rgodf.add(i).read_unaligned();
        let any_inst = (obj.dw_type & DIDFT_ANYINSTANCE) == DIDFT_ANYINSTANCE;
        let inst = ((obj.dw_type >> 8) & 0xFFFF) as u8;
        let g = if obj.pguid.is_null() { None } else { Some(obj.pguid.read_unaligned()) };
        let kind = match g {
            Some(x) if x == GUID_XAXIS => Some(ObjKind::Axis(AxisId::X)),
            Some(x) if x == GUID_YAXIS => Some(ObjKind::Axis(AxisId::Y)),
            Some(x) if x == GUID_ZAXIS => Some(ObjKind::Axis(AxisId::Z)),
            Some(x) if x == GUID_RXAXIS => Some(ObjKind::Axis(AxisId::Rx)),
            Some(x) if x == GUID_RYAXIS => Some(ObjKind::Axis(AxisId::Ry)),
            Some(x) if x == GUID_RZAXIS => Some(ObjKind::Axis(AxisId::Rz)),
            Some(x) if x == GUID_SLIDER => {
                let s = if any_inst {
                    let cur = next_slider;
                    next_slider = next_slider.saturating_add(1);
                    cur
                } else {
                    inst
                };
                match s {
                    0 => Some(ObjKind::Axis(AxisId::Slider0)),
                    1 => Some(ObjKind::Axis(AxisId::Slider1)),
                    _ => None,
                }
            }
            Some(x) if x == GUID_POV => {
                let p = if any_inst {
                    let cur = next_pov;
                    next_pov = next_pov.saturating_add(1);
                    cur
                } else {
                    inst
                };
                Some(ObjKind::Pov(p))
            }
            Some(x) if x == GUID_BUTTON => {
                let b = if any_inst {
                    let cur = next_btn;
                    next_btn = next_btn.saturating_add(1);
                    cur
                } else {
                    inst
                };
                Some(ObjKind::Button(b))
            }
            _ => {
                if obj.dw_type & DIDFT_POV != 0 {
                    let p = if any_inst {
                        let cur = next_pov;
                        next_pov = next_pov.saturating_add(1);
                        cur
                    } else {
                        inst
                    };
                    Some(ObjKind::Pov(p))
                } else if obj.dw_type & DIDFT_BUTTON != 0 {
                    let b = if any_inst {
                        let cur = next_btn;
                        next_btn = next_btn.saturating_add(1);
                        cur
                    } else {
                        inst
                    };
                    Some(ObjKind::Button(b))
                } else if obj.dw_type & DIDFT_AXIS != 0 {
                    let a = if any_inst {
                        let cur = next_any_axis;
                        next_any_axis = next_any_axis.saturating_add(1);
                        cur
                    } else {
                        inst
                    };
                    match a {
                        0 => Some(ObjKind::Axis(AxisId::X)),
                        1 => Some(ObjKind::Axis(AxisId::Y)),
                        2 => Some(ObjKind::Axis(AxisId::Z)),
                        3 => Some(ObjKind::Axis(AxisId::Rx)),
                        4 => Some(ObjKind::Axis(AxisId::Ry)),
                        5 => Some(ObjKind::Axis(AxisId::Slider0)),
                        6 => Some(ObjKind::Axis(AxisId::Slider1)),
                        _ => None,
                    }
                } else {
                    None
                }
            }
        };
        if let Some(kind) = kind {
            let need_bytes = match kind {
                ObjKind::Axis(_) | ObjKind::Pov(_) => 4u32,
                ObjKind::Button(_) => 1u32,
            };
            if obj.dw_ofs.checked_add(need_bytes).is_some_and(|end| end <= df.dw_data_size) {
                out.push(MappedObj { kind, ofs: obj.dw_ofs });
            }
        }
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// IDirectInputDevice8W / IDirectInputDevice8A
// ---------------------------------------------------------------------------

struct DeviceState {
    slot: usize,
    acquired: bool,
    coop_hwnd: usize,
    coop_flags: u32,
    event_handle: usize,
    buffer_size: u32,
    ff_gain: u32,
    autocenter: u32,
    calibration_mode: u32,
    axis_ranges: [(i32, i32); NUM_AXES],
    axis_deadzones: [u32; NUM_AXES],
    axis_saturations: [u32; NUM_AXES],
    axis_cals: [(i32, i32, i32); NUM_AXES],
    app_data: HashMap<u32, usize>,
    data_size: u32,
    mappings: Vec<MappedObj>,
    last_pad: Option<Gamepad>,
    seq: u32,
    events: VecDeque<DIDEVICEOBJECTDATA>,
}

impl DeviceState {
    fn new(slot: usize) -> Self {
        Self {
            slot,
            acquired: false,
            coop_hwnd: 0,
            coop_flags: 0,
            event_handle: 0,
            buffer_size: 0,
            ff_gain: 10_000,
            autocenter: DIPROPAUTOCENTER_ON,
            calibration_mode: DIPROPCALIBRATIONMODE_COOKED,
            axis_ranges: [(DEFAULT_AXIS_MIN, DEFAULT_AXIS_MAX); NUM_AXES],
            axis_deadzones: [0; NUM_AXES],
            axis_saturations: [10000; NUM_AXES],
            axis_cals: [(DEFAULT_AXIS_MIN, 32_767, DEFAULT_AXIS_MAX); NUM_AXES],
            app_data: HashMap::new(),
            data_size: 0,
            mappings: Vec::new(),
            last_pad: None,
            seq: 1,
            events: VecDeque::new(),
        }
    }

    fn resolve_axis_indices(&self, dw_obj: u32, dw_how: u32) -> Option<Vec<usize>> {
        match dw_how {
            DIPH_DEVICE => Some((0..NUM_AXES).collect()),
            DIPH_BYOFFSET => {
                // Match against current data format first, else standard DIJOYSTATE offsets.
                if let Some(m) = self.mappings.iter().find(|m| m.ofs == dw_obj) {
                    if let ObjKind::Axis(a) = m.kind {
                        return Some(vec![a as usize]);
                    }
                    return None;
                }
                let aid = match dw_obj {
                    0 => AxisId::X,
                    4 => AxisId::Y,
                    8 => AxisId::Z,
                    12 => AxisId::Rx,
                    16 => AxisId::Ry,
                    20 => AxisId::Rz,
                    24 => AxisId::Slider0,
                    28 => AxisId::Slider1,
                    _ => return None,
                };
                Some(vec![aid as usize])
            }
            DIPH_BYID => {
                let objs = native_objects();
                objs.iter().find(|o| o.dw_type == dw_obj).and_then(|o| match o.kind {
                    ObjKind::Axis(a) => Some(vec![a as usize]),
                    _ => None,
                })
            }
            _ => None,
        }
    }

    fn offset_for_kind(&self, kind: ObjKind, fallback: u32) -> u32 {
        self.mappings.iter().find(|m| m.kind == kind).map(|m| m.ofs).unwrap_or(fallback)
    }

    fn sample_axis(&self, pad: &Gamepad, aid: AxisId) -> i32 {
        if self.calibration_mode == DIPROPCALIBRATIONMODE_RAW {
            axis_raw_value(pad, aid)
        } else {
            let i = aid as usize;
            axis_value_cooked(
                pad,
                aid,
                self.axis_cals[i],
                self.axis_ranges[i],
                self.axis_deadzones[i],
                self.axis_saturations[i],
            )
        }
    }

    fn sample_buffered(&mut self, pad: Gamepad) -> bool {
        if self.buffer_size == 0 || self.mappings.is_empty() {
            self.last_pad = Some(pad);
            return false;
        }
        let prev = match self.last_pad {
            Some(p) => p,
            None => {
                self.last_pad = Some(pad);
                return false;
            }
        };
        if prev == pad {
            return false;
        }
        let ts = crate::user32::tick_ms();
        let mut overflow = false;
        let cap = self.buffer_size as usize;
        let mappings = self.mappings.clone();
        for m in mappings {
            let (old_v, new_v) = match m.kind {
                ObjKind::Axis(a) => {
                    let o = self.sample_axis(&prev, a) as u32;
                    let n = self.sample_axis(&pad, a) as u32;
                    (o, n)
                }
                ObjKind::Pov(0) => (dpad_pov(prev.buttons), dpad_pov(pad.buttons)),
                ObjKind::Pov(_) => (u32::MAX, u32::MAX),
                ObjKind::Button(b) => {
                    let o = if button_pressed(&prev, b) { 0x80 } else { 0 };
                    let n = if button_pressed(&pad, b) { 0x80 } else { 0 };
                    (o, n)
                }
            };
            if old_v != new_v {
                let seq = self.seq;
                self.seq = self.seq.wrapping_add(1).max(1);
                if self.events.len() >= cap {
                    self.events.pop_front();
                    overflow = true;
                }
                self.events.push_back(DIDEVICEOBJECTDATA {
                    dw_ofs: m.ofs,
                    dw_data: new_v,
                    dw_time_stamp: ts,
                    dw_sequence: seq,
                    u_app_data: self.app_data.get(&m.ofs).copied().unwrap_or(0),
                });
            }
        }
        self.last_pad = Some(pad);
        overflow
    }
}

#[repr(C)]
struct DeviceVTable {
    query_interface: unsafe extern "win64" fn(*mut DeviceObj, *const Guid, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut DeviceObj) -> u32,
    release: unsafe extern "win64" fn(*mut DeviceObj) -> u32,
    get_capabilities: unsafe extern "win64" fn(*mut DeviceObj, *mut DIDEVCAPS) -> HRESULT,
    enum_objects: unsafe extern "win64" fn(*mut DeviceObj, usize, *mut c_void, u32) -> HRESULT,
    get_property: unsafe extern "win64" fn(*mut DeviceObj, *const Guid, *mut DIPROPHEADER) -> HRESULT,
    set_property: unsafe extern "win64" fn(*mut DeviceObj, *const Guid, *const DIPROPHEADER) -> HRESULT,
    acquire: unsafe extern "win64" fn(*mut DeviceObj) -> HRESULT,
    unacquire: unsafe extern "win64" fn(*mut DeviceObj) -> HRESULT,
    get_device_state: unsafe extern "win64" fn(*mut DeviceObj, u32, *mut c_void) -> HRESULT,
    get_device_data: unsafe extern "win64" fn(*mut DeviceObj, u32, *mut c_void, *mut u32, u32) -> HRESULT,
    set_data_format: unsafe extern "win64" fn(*mut DeviceObj, *const DIDATAFORMAT) -> HRESULT,
    set_event_notification: unsafe extern "win64" fn(*mut DeviceObj, usize) -> HRESULT,
    set_cooperative_level: unsafe extern "win64" fn(*mut DeviceObj, usize, u32) -> HRESULT,
    get_object_info: unsafe extern "win64" fn(*mut DeviceObj, *mut u8, u32, u32) -> HRESULT,
    get_device_info: unsafe extern "win64" fn(*mut DeviceObj, *mut u8) -> HRESULT,
    run_control_panel: unsafe extern "win64" fn(*mut DeviceObj, usize, u32) -> HRESULT,
    initialize: unsafe extern "win64" fn(*mut DeviceObj, usize, u32, *const Guid) -> HRESULT,
    create_effect: unsafe extern "win64" fn(*mut DeviceObj, *const Guid, *const c_void, *mut *mut c_void, *mut c_void) -> HRESULT,
    enum_effects: unsafe extern "win64" fn(*mut DeviceObj, usize, *mut c_void, u32) -> HRESULT,
    get_effect_info: unsafe extern "win64" fn(*mut DeviceObj, *mut c_void, *const Guid) -> HRESULT,
    get_force_feedback_state: unsafe extern "win64" fn(*mut DeviceObj, *mut u32) -> HRESULT,
    send_force_feedback_command: unsafe extern "win64" fn(*mut DeviceObj, u32) -> HRESULT,
    enum_created_effect_objects: unsafe extern "win64" fn(*mut DeviceObj, usize, *mut c_void, u32) -> HRESULT,
    escape: unsafe extern "win64" fn(*mut DeviceObj, *mut c_void) -> HRESULT,
    poll: unsafe extern "win64" fn(*mut DeviceObj) -> HRESULT,
    send_device_data: unsafe extern "win64" fn(*mut DeviceObj, u32, *const c_void, *mut u32, u32) -> HRESULT,
    enum_effects_in_file: unsafe extern "win64" fn(*mut DeviceObj, *const u8, usize, *mut c_void, u32) -> HRESULT,
    write_effect_to_file: unsafe extern "win64" fn(*mut DeviceObj, *const u8, u32, *const c_void, u32) -> HRESULT,
    build_action_map: unsafe extern "win64" fn(*mut DeviceObj, *mut c_void, *const u8, u32) -> HRESULT,
    set_action_map: unsafe extern "win64" fn(*mut DeviceObj, *mut c_void, *const u8, u32) -> HRESULT,
    get_image_info: unsafe extern "win64" fn(*mut DeviceObj, *mut c_void) -> HRESULT,
}

#[repr(C)]
struct DeviceObj {
    vtbl: *const DeviceVTable,
    refs: AtomicU32,
    wide: bool,
    state: Mutex<DeviceState>,
}

static DEV_VTBL: DeviceVTable = DeviceVTable {
    query_interface: dev_query_interface,
    add_ref: dev_add_ref,
    release: dev_release,
    get_capabilities: dev_get_capabilities,
    enum_objects: dev_enum_objects,
    get_property: dev_get_property,
    set_property: dev_set_property,
    acquire: dev_acquire,
    unacquire: dev_unacquire,
    get_device_state: dev_get_device_state,
    get_device_data: dev_get_device_data,
    set_data_format: dev_set_data_format,
    set_event_notification: dev_set_event_notification,
    set_cooperative_level: dev_set_cooperative_level,
    get_object_info: dev_get_object_info,
    get_device_info: dev_get_device_info,
    run_control_panel: dev_run_control_panel,
    initialize: dev_initialize,
    create_effect: dev_create_effect,
    enum_effects: dev_enum_effects,
    get_effect_info: dev_get_effect_info,
    get_force_feedback_state: dev_get_force_feedback_state,
    send_force_feedback_command: dev_send_force_feedback_command,
    enum_created_effect_objects: dev_enum_created_effect_objects,
    escape: dev_escape,
    poll: dev_poll,
    send_device_data: dev_send_device_data,
    enum_effects_in_file: dev_enum_effects_in_file,
    write_effect_to_file: dev_write_effect_to_file,
    build_action_map: dev_build_action_map,
    set_action_map: dev_set_action_map,
    get_image_info: dev_get_image_info,
};

fn new_device(slot: usize, wide: bool) -> *mut c_void {
    crate::com::register_vtbl_struct(&DEV_VTBL);
    Box::into_raw(Box::new(DeviceObj {
        vtbl: &DEV_VTBL,
        refs: AtomicU32::new(1),
        wide,
        state: Mutex::new(DeviceState::new(slot)),
    })) as *mut c_void
}

unsafe extern "win64" fn dev_query_interface(this: *mut DeviceObj, iid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() || iid.is_null() {
        return E_POINTER;
    }
    let g = iid.read_unaligned();
    let ok = g == IID_IUNKNOWN
        || ((*this).wide && g == IID_IDIRECTINPUTDEVICE8W)
        || (!(*this).wide && g == IID_IDIRECTINPUTDEVICE8A);
    if ok {
        dev_add_ref(this);
        out.write_unaligned(this as *mut c_void);
        S_OK
    } else {
        out.write_unaligned(std::ptr::null_mut());
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn dev_add_ref(this: *mut DeviceObj) -> u32 {
    (*this).refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn dev_release(this: *mut DeviceObj) -> u32 {
    let n = (*this).refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if n == 0 {
        drop(Box::from_raw(this));
    }
    n
}

unsafe extern "win64" fn dev_get_capabilities(this: *mut DeviceObj, caps: *mut DIDEVCAPS) -> HRESULT {
    if caps.is_null() {
        return E_POINTER;
    }
    let dw_size = (caps as *const u32).read_unaligned();
    if dw_size != std::mem::size_of::<DIDEVCAPS>() as u32 && dw_size != 24 {
        return DIERR_INVALIDPARAM;
    }
    let slot = (*this).state.lock().unwrap().slot;
    let attached = xinput1_4::slot_state(slot).is_some();
    let out = DIDEVCAPS {
        dw_size,
        dw_flags: if attached { DIDC_ATTACHED } else { 0 },
        dw_dev_type: GAMEPAD_DEVTYPE,
        dw_axes: 7,
        dw_buttons: NUM_BUTTONS as u32,
        dw_povs: 1,
        ..Default::default()
    };
    std::ptr::copy_nonoverlapping(&out as *const DIDEVCAPS as *const u8, caps as *mut u8, dw_size as usize);
    DI_OK
}

unsafe extern "win64" fn dev_enum_objects(this: *mut DeviceObj, cb: usize, pv_ref: *mut c_void, flags: u32) -> HRESULT {
    if cb == 0 {
        return DIERR_INVALIDPARAM;
    }
    let wide = (*this).wide;
    let type_mask = flags & 0xFF;
    let items: Vec<(NativeObjDesc, u32)> = {
        let st = (*this).state.lock().unwrap();
        native_objects()
            .into_iter()
            .filter(|o| type_mask == DIDFT_ALL || (o.dw_type & type_mask) != 0)
            .map(|o| {
                let ofs = st.offset_for_kind(o.kind, o.default_ofs);
                (o, ofs)
            })
            .collect()
    };
    for (desc, ofs) in items {
        let cont = if wide {
            let mut inst = make_obj_instance_w(&desc, ofs);
            crate::call_guest8(cb, (&mut inst as *mut DIDEVICEOBJECTINSTANCEW) as u64, pv_ref as u64, 0, 0, 0, 0, 0, 0) as i32
        } else {
            let mut inst = make_obj_instance_a(&desc, ofs);
            crate::call_guest8(cb, (&mut inst as *mut DIDEVICEOBJECTINSTANCEA) as u64, pv_ref as u64, 0, 0, 0, 0, 0, 0) as i32
        };
        if cont == DIENUM_STOP {
            break;
        }
    }
    DI_OK
}

unsafe extern "win64" fn dev_get_property(this: *mut DeviceObj, prop: *const Guid, diph: *mut DIPROPHEADER) -> HRESULT {
    if diph.is_null() {
        return E_POINTER;
    }
    let hdr = diph.read_unaligned();
    if hdr.dw_header_size != std::mem::size_of::<DIPROPHEADER>() as u32 {
        return DIERR_INVALIDPARAM;
    }
    let st = (*this).state.lock().unwrap();
    match prop as usize {
        DIPROP_RANGE => {
            if hdr.dw_size != std::mem::size_of::<DIPROPRANGE>() as u32 {
                return DIERR_INVALIDPARAM;
            }
            let Some(axes) = st.resolve_axis_indices(hdr.dw_obj, hdr.dw_how) else {
                return DIERR_OBJECTNOTFOUND;
            };
            let (l_min, l_max) = st.axis_ranges[axes[0]];
            (diph as *mut DIPROPRANGE).write_unaligned(DIPROPRANGE { diph: hdr, l_min, l_max });
            DI_OK
        }
        DIPROP_PHYSICALRANGE | DIPROP_LOGICALRANGE => {
            if hdr.dw_size != std::mem::size_of::<DIPROPRANGE>() as u32 {
                return DIERR_INVALIDPARAM;
            }
            if st.resolve_axis_indices(hdr.dw_obj, hdr.dw_how).is_none() {
                return DIERR_OBJECTNOTFOUND;
            }
            (diph as *mut DIPROPRANGE).write_unaligned(DIPROPRANGE {
                diph: hdr,
                l_min: DEFAULT_AXIS_MIN,
                l_max: DEFAULT_AXIS_MAX,
            });
            DI_OK
        }
        DIPROP_DEADZONE => {
            if hdr.dw_size != std::mem::size_of::<DIPROPDWORD>() as u32 {
                return DIERR_INVALIDPARAM;
            }
            let Some(axes) = st.resolve_axis_indices(hdr.dw_obj, hdr.dw_how) else {
                return DIERR_OBJECTNOTFOUND;
            };
            (diph as *mut DIPROPDWORD).write_unaligned(DIPROPDWORD { diph: hdr, dw_data: st.axis_deadzones[axes[0]] });
            DI_OK
        }
        DIPROP_SATURATION => {
            if hdr.dw_size != std::mem::size_of::<DIPROPDWORD>() as u32 {
                return DIERR_INVALIDPARAM;
            }
            let Some(axes) = st.resolve_axis_indices(hdr.dw_obj, hdr.dw_how) else {
                return DIERR_OBJECTNOTFOUND;
            };
            (diph as *mut DIPROPDWORD).write_unaligned(DIPROPDWORD { diph: hdr, dw_data: st.axis_saturations[axes[0]] });
            DI_OK
        }
        DIPROP_AXISMODE => {
            if hdr.dw_size != std::mem::size_of::<DIPROPDWORD>() as u32 || hdr.dw_how != DIPH_DEVICE {
                return DIERR_INVALIDPARAM;
            }
            (diph as *mut DIPROPDWORD).write_unaligned(DIPROPDWORD { diph: hdr, dw_data: DIPROPAXISMODE_ABS });
            DI_OK
        }
        DIPROP_BUFFERSIZE => {
            if hdr.dw_size != std::mem::size_of::<DIPROPDWORD>() as u32 || hdr.dw_how != DIPH_DEVICE {
                return DIERR_INVALIDPARAM;
            }
            (diph as *mut DIPROPDWORD).write_unaligned(DIPROPDWORD { diph: hdr, dw_data: st.buffer_size });
            DI_OK
        }
        DIPROP_GRANULARITY => {
            if hdr.dw_size != std::mem::size_of::<DIPROPDWORD>() as u32 {
                return DIERR_INVALIDPARAM;
            }
            (diph as *mut DIPROPDWORD).write_unaligned(DIPROPDWORD { diph: hdr, dw_data: 1 });
            DI_OK
        }
        DIPROP_VIDPID => {
            if hdr.dw_size != std::mem::size_of::<DIPROPDWORD>() as u32 || hdr.dw_how != DIPH_DEVICE {
                return DIERR_INVALIDPARAM;
            }
            (diph as *mut DIPROPDWORD).write_unaligned(DIPROPDWORD { diph: hdr, dw_data: GAMEPAD_VIDPID });
            DI_OK
        }
        DIPROP_JOYSTICKID => {
            if hdr.dw_size != std::mem::size_of::<DIPROPDWORD>() as u32 || hdr.dw_how != DIPH_DEVICE {
                return DIERR_INVALIDPARAM;
            }
            (diph as *mut DIPROPDWORD).write_unaligned(DIPROPDWORD { diph: hdr, dw_data: st.slot as u32 });
            DI_OK
        }
        DIPROP_FFGAIN => {
            if hdr.dw_size != std::mem::size_of::<DIPROPDWORD>() as u32 || hdr.dw_how != DIPH_DEVICE {
                return DIERR_INVALIDPARAM;
            }
            (diph as *mut DIPROPDWORD).write_unaligned(DIPROPDWORD { diph: hdr, dw_data: st.ff_gain });
            DI_OK
        }
        DIPROP_FFLOAD => {
            if hdr.dw_size != std::mem::size_of::<DIPROPDWORD>() as u32 || hdr.dw_how != DIPH_DEVICE {
                return DIERR_INVALIDPARAM;
            }
            (diph as *mut DIPROPDWORD).write_unaligned(DIPROPDWORD { diph: hdr, dw_data: 0 });
            DI_OK
        }
        DIPROP_AUTOCENTER => {
            if hdr.dw_size != std::mem::size_of::<DIPROPDWORD>() as u32 || hdr.dw_how != DIPH_DEVICE {
                return DIERR_INVALIDPARAM;
            }
            (diph as *mut DIPROPDWORD).write_unaligned(DIPROPDWORD { diph: hdr, dw_data: st.autocenter });
            DI_OK
        }
        DIPROP_CALIBRATIONMODE => {
            if hdr.dw_size != std::mem::size_of::<DIPROPDWORD>() as u32 || hdr.dw_how != DIPH_DEVICE {
                return DIERR_INVALIDPARAM;
            }
            (diph as *mut DIPROPDWORD).write_unaligned(DIPROPDWORD { diph: hdr, dw_data: st.calibration_mode });
            DI_OK
        }
        DIPROP_CALIBRATION => {
            if hdr.dw_size != std::mem::size_of::<DIPROPCAL>() as u32 {
                return DIERR_INVALIDPARAM;
            }
            let Some(axes) = st.resolve_axis_indices(hdr.dw_obj, hdr.dw_how) else {
                return DIERR_OBJECTNOTFOUND;
            };
            let (l_min, l_center, l_max) = st.axis_cals[axes[0]];
            (diph as *mut DIPROPCAL).write_unaligned(DIPROPCAL { diph: hdr, l_min, l_center, l_max });
            DI_OK
        }
        DIPROP_GUIDANDPATH => {
            if hdr.dw_size != std::mem::size_of::<DIPROPGUIDANDPATH>() as u32 || hdr.dw_how != DIPH_DEVICE {
                return DIERR_INVALIDPARAM;
            }
            // Expose standard Xbox/XInput HID path with `IG_` marker so games/SDL
            // can pair DirectInput devices with their XInput slots without double-input.
            let hid_path = format!(
                "\\\\?\\hid#vid_045e&pid_028e&ig_{:02}#7&{:08x}&0&0000#{{4d1e55b2-f16f-11cf-88cb-001111000030}}",
                st.slot,
                0x1000 + st.slot
            );
            let mut out = DIPROPGUIDANDPATH {
                diph: hdr,
                guid_class: crate::misc::hid::GUID_DEVINTERFACE_HID,
                wsz_path: [0; MAX_PATH],
            };
            fill_wide_fixed(&mut out.wsz_path, &hid_path);
            (diph as *mut DIPROPGUIDANDPATH).write_unaligned(out);
            DI_OK
        }
        DIPROP_INSTANCENAME | DIPROP_PRODUCTNAME | DIPROP_GETPORTDISPLAYNAME | DIPROP_TYPENAME => {
            if hdr.dw_size != std::mem::size_of::<DIPROPSTRING>() as u32 || hdr.dw_how != DIPH_DEVICE {
                return DIERR_INVALIDPARAM;
            }
            let text = match prop as usize {
                DIPROP_INSTANCENAME => slot_instance_name(st.slot),
                DIPROP_PRODUCTNAME => PRODUCT_NAME.to_string(),
                DIPROP_GETPORTDISPLAYNAME => format!("GameController Port {}", st.slot + 1),
                _ => "VID_045E&PID_028E".to_string(),
            };
            let mut out = DIPROPSTRING { diph: hdr, wsz: [0; MAX_PATH] };
            fill_wide_fixed(&mut out.wsz, &text);
            (diph as *mut DIPROPSTRING).write_unaligned(out);
            DI_OK
        }
        DIPROP_KEYNAME => {
            if hdr.dw_size != std::mem::size_of::<DIPROPSTRING>() as u32 {
                return DIERR_INVALIDPARAM;
            }
            let objs = native_objects();
            let found = match hdr.dw_how {
                DIPH_BYID => objs.iter().find(|o| o.dw_type == hdr.dw_obj).copied(),
                DIPH_BYOFFSET => {
                    if let Some(m) = st.mappings.iter().find(|m| m.ofs == hdr.dw_obj) {
                        objs.iter().find(|o| o.kind == m.kind).copied()
                    } else {
                        objs.iter().find(|o| o.default_ofs == hdr.dw_obj).copied()
                    }
                }
                _ => return DIERR_INVALIDPARAM,
            };
            let Some(desc) = found else {
                return DIERR_OBJECTNOTFOUND;
            };
            let mut out = DIPROPSTRING { diph: hdr, wsz: [0; MAX_PATH] };
            fill_wide_fixed(&mut out.wsz, desc.name);
            (diph as *mut DIPROPSTRING).write_unaligned(out);
            DI_OK
        }
        DIPROP_APPDATA => {
            if hdr.dw_size != std::mem::size_of::<DIPROPPOINTER>() as u32 {
                return DIERR_INVALIDPARAM;
            }
            let key = match hdr.dw_how {
                DIPH_BYOFFSET => hdr.dw_obj,
                DIPH_BYID => {
                    let objs = native_objects();
                    let Some(desc) = objs.iter().find(|o| o.dw_type == hdr.dw_obj) else {
                        return DIERR_OBJECTNOTFOUND;
                    };
                    st.offset_for_kind(desc.kind, desc.default_ofs)
                }
                _ => return DIERR_INVALIDPARAM,
            };
            let u_data = st.app_data.get(&key).copied().unwrap_or(0);
            (diph as *mut DIPROPPOINTER).write_unaligned(DIPROPPOINTER { diph: hdr, u_data });
            DI_OK
        }
        DIPROP_USERNAME => {
            // Per Microsoft Learn: if no user is currently assigned, wsz is empty and S_FALSE is returned.
            if hdr.dw_size != std::mem::size_of::<DIPROPSTRING>() as u32 || hdr.dw_how != DIPH_DEVICE {
                return DIERR_INVALIDPARAM;
            }
            let out = DIPROPSTRING { diph: hdr, wsz: [0; MAX_PATH] };
            (diph as *mut DIPROPSTRING).write_unaligned(out);
            DI_NOEFFECT // S_FALSE
        }
        _ => {
            // Per Microsoft Learn IDirectInputDevice8::GetProperty: unsupported/keyboard-only
            // property GUIDs (e.g. DIPROP_SCANCODE, DIPROP_CPOINTS) return DIERR_UNSUPPORTED.
            DIERR_UNSUPPORTED
        }
    }
}

unsafe extern "win64" fn dev_set_property(this: *mut DeviceObj, prop: *const Guid, diph: *const DIPROPHEADER) -> HRESULT {
    if diph.is_null() {
        return E_POINTER;
    }
    let hdr = diph.read_unaligned();
    if hdr.dw_header_size != std::mem::size_of::<DIPROPHEADER>() as u32 {
        return DIERR_INVALIDPARAM;
    }
    let mut st = (*this).state.lock().unwrap();
    match prop as usize {
        DIPROP_RANGE => {
            if hdr.dw_size != std::mem::size_of::<DIPROPRANGE>() as u32 {
                return DIERR_INVALIDPARAM;
            }
            let r = (diph as *const DIPROPRANGE).read_unaligned();
            if r.l_min >= r.l_max {
                return DIERR_INVALIDPARAM;
            }
            let Some(axes) = st.resolve_axis_indices(hdr.dw_obj, hdr.dw_how) else {
                return DIERR_OBJECTNOTFOUND;
            };
            for i in axes {
                st.axis_ranges[i] = (r.l_min, r.l_max);
            }
            DI_OK
        }
        DIPROP_DEADZONE => {
            if hdr.dw_size != std::mem::size_of::<DIPROPDWORD>() as u32 {
                return DIERR_INVALIDPARAM;
            }
            let v = (diph as *const DIPROPDWORD).read_unaligned().dw_data;
            if v > 10000 {
                return DIERR_INVALIDPARAM;
            }
            let Some(axes) = st.resolve_axis_indices(hdr.dw_obj, hdr.dw_how) else {
                return DIERR_OBJECTNOTFOUND;
            };
            for i in axes {
                st.axis_deadzones[i] = v;
            }
            DI_OK
        }
        DIPROP_SATURATION => {
            if hdr.dw_size != std::mem::size_of::<DIPROPDWORD>() as u32 {
                return DIERR_INVALIDPARAM;
            }
            let v = (diph as *const DIPROPDWORD).read_unaligned().dw_data;
            if v > 10000 {
                return DIERR_INVALIDPARAM;
            }
            let Some(axes) = st.resolve_axis_indices(hdr.dw_obj, hdr.dw_how) else {
                return DIERR_OBJECTNOTFOUND;
            };
            for i in axes {
                st.axis_saturations[i] = v;
            }
            DI_OK
        }
        DIPROP_AXISMODE => {
            if hdr.dw_size != std::mem::size_of::<DIPROPDWORD>() as u32 || hdr.dw_how != DIPH_DEVICE {
                return DIERR_INVALIDPARAM;
            }
            let mode = (diph as *const DIPROPDWORD).read_unaligned().dw_data;
            if mode == DIPROPAXISMODE_ABS {
                DI_OK
            } else {
                DIERR_UNSUPPORTED
            }
        }
        DIPROP_BUFFERSIZE => {
            if hdr.dw_size != std::mem::size_of::<DIPROPDWORD>() as u32 || hdr.dw_how != DIPH_DEVICE {
                return DIERR_INVALIDPARAM;
            }
            let sz = (diph as *const DIPROPDWORD).read_unaligned().dw_data.min(1024);
            st.buffer_size = sz;
            if sz == 0 {
                st.events.clear();
            } else {
                while st.events.len() > sz as usize {
                    st.events.pop_front();
                }
            }
            DI_OK
        }
        DIPROP_FFGAIN => {
            if hdr.dw_size != std::mem::size_of::<DIPROPDWORD>() as u32 || hdr.dw_how != DIPH_DEVICE {
                return DIERR_INVALIDPARAM;
            }
            let v = (diph as *const DIPROPDWORD).read_unaligned().dw_data;
            if v > 10_000 {
                return DIERR_INVALIDPARAM;
            }
            st.ff_gain = v;
            DI_PROPNOEFFECT
        }
        DIPROP_AUTOCENTER => {
            if hdr.dw_size != std::mem::size_of::<DIPROPDWORD>() as u32 || hdr.dw_how != DIPH_DEVICE {
                return DIERR_INVALIDPARAM;
            }
            let v = (diph as *const DIPROPDWORD).read_unaligned().dw_data;
            if v != DIPROPAUTOCENTER_OFF && v != DIPROPAUTOCENTER_ON {
                return DIERR_INVALIDPARAM;
            }
            st.autocenter = v;
            DI_PROPNOEFFECT
        }
        DIPROP_CALIBRATIONMODE => {
            if hdr.dw_size != std::mem::size_of::<DIPROPDWORD>() as u32 || hdr.dw_how != DIPH_DEVICE {
                return DIERR_INVALIDPARAM;
            }
            let v = (diph as *const DIPROPDWORD).read_unaligned().dw_data;
            if v != DIPROPCALIBRATIONMODE_COOKED && v != DIPROPCALIBRATIONMODE_RAW {
                return DIERR_INVALIDPARAM;
            }
            st.calibration_mode = v;
            DI_OK
        }
        DIPROP_CALIBRATION => {
            if hdr.dw_size != std::mem::size_of::<DIPROPCAL>() as u32 {
                return DIERR_INVALIDPARAM;
            }
            let c = (diph as *const DIPROPCAL).read_unaligned();
            if c.l_min >= c.l_max || c.l_center < c.l_min || c.l_center > c.l_max {
                return DIERR_INVALIDPARAM;
            }
            let Some(axes) = st.resolve_axis_indices(hdr.dw_obj, hdr.dw_how) else {
                return DIERR_OBJECTNOTFOUND;
            };
            for i in axes {
                st.axis_cals[i] = (c.l_min, c.l_center, c.l_max);
            }
            DI_OK
        }
        DIPROP_APPDATA => {
            if hdr.dw_size != std::mem::size_of::<DIPROPPOINTER>() as u32 {
                return DIERR_INVALIDPARAM;
            }
            let v = (diph as *const DIPROPPOINTER).read_unaligned().u_data;
            let key = match hdr.dw_how {
                DIPH_BYOFFSET => hdr.dw_obj,
                DIPH_BYID => {
                    let objs = native_objects();
                    let Some(desc) = objs.iter().find(|o| o.dw_type == hdr.dw_obj) else {
                        return DIERR_OBJECTNOTFOUND;
                    };
                    st.offset_for_kind(desc.kind, desc.default_ofs)
                }
                _ => return DIERR_INVALIDPARAM,
            };
            st.app_data.insert(key, v);
            DI_OK
        }
        _ => {
            // Per Microsoft Learn IDirectInputDevice8::SetProperty: read-only or unsupported
            // properties return DIERR_UNSUPPORTED.
            DIERR_UNSUPPORTED
        }
    }
}

unsafe extern "win64" fn dev_acquire(this: *mut DeviceObj) -> HRESULT {
    let mut st = (*this).state.lock().unwrap();
    if st.data_size == 0 {
        return DIERR_INVALIDPARAM;
    }
    let Some((pad, _)) = xinput1_4::slot_state(st.slot) else {
        st.acquired = false;
        return DIERR_INPUTLOST;
    };
    if st.acquired {
        return DI_NOEFFECT;
    }
    st.acquired = true;
    st.last_pad = Some(pad);
    DI_OK
}

unsafe extern "win64" fn dev_unacquire(this: *mut DeviceObj) -> HRESULT {
    let mut st = (*this).state.lock().unwrap();
    if st.acquired {
        st.acquired = false;
        DI_OK
    } else {
        DI_NOEFFECT
    }
}

unsafe extern "win64" fn dev_poll(this: *mut DeviceObj) -> HRESULT {
    let mut st = (*this).state.lock().unwrap();
    if !st.acquired {
        return DIERR_NOTACQUIRED;
    }
    let Some((pad, _)) = xinput1_4::slot_state(st.slot) else {
        st.acquired = false;
        return DIERR_INPUTLOST;
    };
    st.sample_buffered(pad);
    DI_OK
}

unsafe extern "win64" fn dev_get_device_state(this: *mut DeviceObj, cb_data: u32, lpv_data: *mut c_void) -> HRESULT {
    if lpv_data.is_null() {
        return E_POINTER;
    }
    let mut st = (*this).state.lock().unwrap();
    if st.data_size == 0 || cb_data != st.data_size {
        return DIERR_INVALIDPARAM;
    }
    if !st.acquired {
        return DIERR_NOTACQUIRED;
    }
    let Some((pad, _)) = xinput1_4::slot_state(st.slot) else {
        st.acquired = false;
        return DIERR_INPUTLOST;
    };
    st.sample_buffered(pad);
    let base = lpv_data as *mut u8;
    std::ptr::write_bytes(base, 0, cb_data as usize);
    for m in &st.mappings {
        match m.kind {
            ObjKind::Axis(a) => {
                let v = st.sample_axis(&pad, a);
                (base.add(m.ofs as usize) as *mut i32).write_unaligned(v);
            }
            ObjKind::Pov(p) => {
                let v = if p == 0 { dpad_pov(pad.buttons) } else { u32::MAX };
                (base.add(m.ofs as usize) as *mut u32).write_unaligned(v);
            }
            ObjKind::Button(b) => {
                let v: u8 = if button_pressed(&pad, b) { 0x80 } else { 0x00 };
                base.add(m.ofs as usize).write_unaligned(v);
            }
        }
    }
    DI_OK
}

unsafe extern "win64" fn dev_get_device_data(
    this: *mut DeviceObj,
    cb_object_data: u32,
    rgdod: *mut c_void,
    pdw_in_out: *mut u32,
    flags: u32,
) -> HRESULT {
    if pdw_in_out.is_null() {
        return E_POINTER;
    }
    if cb_object_data != 16 && cb_object_data != std::mem::size_of::<DIDEVICEOBJECTDATA>() as u32 {
        return DIERR_INVALIDPARAM;
    }
    let mut st = (*this).state.lock().unwrap();
    if !st.acquired {
        return DIERR_NOTACQUIRED;
    }
    let Some((pad, _)) = xinput1_4::slot_state(st.slot) else {
        st.acquired = false;
        return DIERR_INPUTLOST;
    };
    let overflow = st.sample_buffered(pad);
    let max_items = pdw_in_out.read_unaligned() as usize;
    let count = if max_items == usize::MAX { st.events.len() } else { max_items.min(st.events.len()) };
    let peek = flags & DIGDD_PEEK != 0;
    if !rgdod.is_null() {
        let dst = rgdod as *mut u8;
        for i in 0..count {
            let item = if peek { st.events[i] } else { st.events.pop_front().unwrap() };
            let slot_ptr = dst.add(i * cb_object_data as usize);
            std::ptr::copy_nonoverlapping(&item as *const DIDEVICEOBJECTDATA as *const u8, slot_ptr, cb_object_data as usize);
        }
    } else if !peek {
        for _ in 0..count {
            st.events.pop_front();
        }
    }
    pdw_in_out.write_unaligned(count as u32);
    if overflow { DI_BUFFEROVERFLOW } else { DI_OK }
}

unsafe extern "win64" fn dev_set_data_format(this: *mut DeviceObj, lpdf: *const DIDATAFORMAT) -> HRESULT {
    if lpdf.is_null() {
        return E_POINTER;
    }
    let df = lpdf.read_unaligned();
    if df.dw_size != std::mem::size_of::<DIDATAFORMAT>() as u32
        || df.dw_obj_size != std::mem::size_of::<DIOBJECTDATAFORMAT>() as u32
        || df.dw_data_size == 0
    {
        return DIERR_INVALIDPARAM;
    }
    let mappings = match parse_data_format(&df) {
        Ok(m) => m,
        Err(hr) => return hr,
    };
    let mut st = (*this).state.lock().unwrap();
    st.data_size = df.dw_data_size;
    st.mappings = mappings;
    DI_OK
}

unsafe extern "win64" fn dev_set_event_notification(this: *mut DeviceObj, h_event: usize) -> HRESULT {
    (*this).state.lock().unwrap().event_handle = h_event;
    DI_OK
}

unsafe extern "win64" fn dev_set_cooperative_level(this: *mut DeviceObj, hwnd: usize, flags: u32) -> HRESULT {
    if (flags & (DISCL_EXCLUSIVE | DISCL_NONEXCLUSIVE)) == (DISCL_EXCLUSIVE | DISCL_NONEXCLUSIVE)
        || (flags & (DISCL_FOREGROUND | DISCL_BACKGROUND)) == (DISCL_FOREGROUND | DISCL_BACKGROUND)
    {
        return DIERR_INVALIDPARAM;
    }
    let mut st = (*this).state.lock().unwrap();
    st.coop_hwnd = hwnd;
    st.coop_flags = flags;
    DI_OK
}

unsafe extern "win64" fn dev_get_object_info(this: *mut DeviceObj, pdidoi: *mut u8, dw_obj: u32, dw_how: u32) -> HRESULT {
    if pdidoi.is_null() {
        return E_POINTER;
    }
    let wide = (*this).wide;
    let dw_size = (pdidoi as *const u32).read_unaligned();
    let valid_sizes = if wide { [576u32, 552] } else { [316u32, 292] };
    if !valid_sizes.contains(&dw_size) {
        return DIERR_INVALIDPARAM;
    }
    let st = (*this).state.lock().unwrap();
    let objs = native_objects();
    let found = match dw_how {
        DIPH_BYID => objs.iter().find(|o| o.dw_type == dw_obj).copied(),
        DIPH_BYOFFSET => {
            if let Some(m) = st.mappings.iter().find(|m| m.ofs == dw_obj) {
                objs.iter().find(|o| o.kind == m.kind).copied()
            } else {
                objs.iter().find(|o| o.default_ofs == dw_obj).copied()
            }
        }
        _ => return DIERR_INVALIDPARAM,
    };
    let Some(desc) = found else {
        return DIERR_OBJECTNOTFOUND;
    };
    let ofs = st.offset_for_kind(desc.kind, desc.default_ofs);
    if wide {
        let mut out = make_obj_instance_w(&desc, ofs);
        out.dw_size = dw_size;
        std::ptr::copy_nonoverlapping(&out as *const DIDEVICEOBJECTINSTANCEW as *const u8, pdidoi, dw_size as usize);
    } else {
        let mut out = make_obj_instance_a(&desc, ofs);
        out.dw_size = dw_size;
        std::ptr::copy_nonoverlapping(&out as *const DIDEVICEOBJECTINSTANCEA as *const u8, pdidoi, dw_size as usize);
    }
    DI_OK
}

unsafe extern "win64" fn dev_get_device_info(this: *mut DeviceObj, pdidi: *mut u8) -> HRESULT {
    if pdidi.is_null() {
        return E_POINTER;
    }
    let wide = (*this).wide;
    let dw_size = (pdidi as *const u32).read_unaligned();
    let valid_sizes = if wide { [1100u32, 1080] } else { [580u32, 560] };
    if !valid_sizes.contains(&dw_size) {
        return DIERR_INVALIDPARAM;
    }
    let slot = (*this).state.lock().unwrap().slot;
    if wide {
        let mut out = make_device_instance_w(slot);
        out.dw_size = dw_size;
        std::ptr::copy_nonoverlapping(&out as *const DIDEVICEINSTANCEW as *const u8, pdidi, dw_size as usize);
    } else {
        let mut out = make_device_instance_a(slot);
        out.dw_size = dw_size;
        std::ptr::copy_nonoverlapping(&out as *const DIDEVICEINSTANCEA as *const u8, pdidi, dw_size as usize);
    }
    DI_OK
}

/// Native-unsupported on macOS/iOS: launches Windows `joy.cpl` Game Controllers Control Panel.
/// Matches Microsoft Learn `IDirectInputDevice8::RunControlPanel`: returns `DIERR_INVALIDPARAM`
/// when reserved `dwFlags != 0` and `DIERR_UNSUPPORTED` when the Control Panel applet is absent.
unsafe extern "win64" fn dev_run_control_panel(_this: *mut DeviceObj, _hwnd: usize, flags: u32) -> HRESULT {
    if flags != 0 {
        return DIERR_INVALIDPARAM;
    }
    DIERR_UNSUPPORTED
}

unsafe extern "win64" fn dev_initialize(_this: *mut DeviceObj, _hinst: usize, version: u32, _rguid: *const Guid) -> HRESULT {
    check_version(version)
}

unsafe extern "win64" fn dev_create_effect(
    _this: *mut DeviceObj,
    _rguid: *const Guid,
    _lpeff: *const c_void,
    ppdeff: *mut *mut c_void,
    _outer: *mut c_void,
) -> HRESULT {
    if !ppdeff.is_null() {
        ppdeff.write_unaligned(std::ptr::null_mut());
    }
    DIERR_UNSUPPORTED
}

unsafe extern "win64" fn dev_enum_effects(_this: *mut DeviceObj, cb: usize, _pv_ref: *mut c_void, _eff_type: u32) -> HRESULT {
    if cb == 0 {
        return DIERR_INVALIDPARAM;
    }
    DI_OK
}

unsafe extern "win64" fn dev_get_effect_info(_this: *mut DeviceObj, _pdei: *mut c_void, _rguid: *const Guid) -> HRESULT {
    DIERR_UNSUPPORTED
}

unsafe extern "win64" fn dev_get_force_feedback_state(_this: *mut DeviceObj, pdw_out: *mut u32) -> HRESULT {
    if !pdw_out.is_null() {
        pdw_out.write_unaligned(0);
    }
    DIERR_UNSUPPORTED
}

unsafe extern "win64" fn dev_send_force_feedback_command(_this: *mut DeviceObj, _flags: u32) -> HRESULT {
    DIERR_UNSUPPORTED
}

unsafe extern "win64" fn dev_enum_created_effect_objects(_this: *mut DeviceObj, cb: usize, _pv_ref: *mut c_void, _fl: u32) -> HRESULT {
    if cb == 0 {
        return DIERR_INVALIDPARAM;
    }
    DI_OK
}

unsafe extern "win64" fn dev_escape(_this: *mut DeviceObj, _pesc: *mut c_void) -> HRESULT {
    DIERR_UNSUPPORTED
}

unsafe extern "win64" fn dev_send_device_data(
    _this: *mut DeviceObj,
    _cb: u32,
    _rgdod: *const c_void,
    pdw_in_out: *mut u32,
    _fl: u32,
) -> HRESULT {
    if !pdw_in_out.is_null() {
        pdw_in_out.write_unaligned(0);
    }
    DIERR_UNSUPPORTED
}

unsafe extern "win64" fn dev_enum_effects_in_file(
    _this: *mut DeviceObj,
    _name: *const u8,
    _cb: usize,
    _pv_ref: *mut c_void,
    _flags: u32,
) -> HRESULT {
    DIERR_UNSUPPORTED
}

unsafe extern "win64" fn dev_write_effect_to_file(
    _this: *mut DeviceObj,
    _name: *const u8,
    _entries: u32,
    _rg: *const c_void,
    _flags: u32,
) -> HRESULT {
    DIERR_UNSUPPORTED
}

unsafe extern "win64" fn dev_build_action_map(_this: *mut DeviceObj, _lpdiaf: *mut c_void, _user: *const u8, _flags: u32) -> HRESULT {
    DIERR_UNSUPPORTED
}

unsafe extern "win64" fn dev_set_action_map(_this: *mut DeviceObj, _map: *mut c_void, _user: *const u8, _flags: u32) -> HRESULT {
    DIERR_UNSUPPORTED
}

unsafe extern "win64" fn dev_get_image_info(_this: *mut DeviceObj, _hdr: *mut c_void) -> HRESULT {
    DIERR_UNSUPPORTED
}

// ---------------------------------------------------------------------------
// IDirectInput8W / IDirectInput8A
// ---------------------------------------------------------------------------

#[repr(C)]
struct VTable {
    query_interface: unsafe extern "win64" fn(*mut Obj, *const Guid, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut Obj) -> u32,
    release: unsafe extern "win64" fn(*mut Obj) -> u32,
    create_device: unsafe extern "win64" fn(*mut Obj, *const Guid, *mut *mut c_void, *mut c_void) -> HRESULT,
    enum_devices: unsafe extern "win64" fn(*mut Obj, u32, usize, *mut c_void, u32) -> HRESULT,
    get_device_status: unsafe extern "win64" fn(*mut Obj, *const Guid) -> HRESULT,
    run_control_panel: unsafe extern "win64" fn(*mut Obj, usize, u32) -> HRESULT,
    initialize: unsafe extern "win64" fn(*mut Obj, usize, u32) -> HRESULT,
    find_device: unsafe extern "win64" fn(*mut Obj, *const Guid, *const u16, *mut Guid) -> HRESULT,
    enum_devices_by_semantics: unsafe extern "win64" fn(*mut Obj, *const u16, *mut c_void, usize, *mut c_void, u32) -> HRESULT,
    configure_devices: unsafe extern "win64" fn(*mut Obj, usize, *mut c_void, u32, *mut c_void) -> HRESULT,
}

#[repr(C)]
struct Obj {
    vtbl: *const VTable,
    refs: AtomicU32,
    wide: bool,
}

static VTBL: VTable = VTable {
    query_interface,
    add_ref,
    release,
    create_device,
    enum_devices,
    get_device_status,
    run_control_panel,
    initialize,
    find_device,
    enum_devices_by_semantics,
    configure_devices,
};

fn new_obj(wide: bool) -> *mut c_void {
    crate::com::register_vtbl_struct(&VTBL);
    Box::into_raw(Box::new(Obj { vtbl: &VTBL, refs: AtomicU32::new(1), wide })) as *mut c_void
}

fn is_di8(iid: &Guid) -> bool {
    *iid == IID_IDIRECTINPUT8W || *iid == IID_IDIRECTINPUT8A || *iid == IID_IUNKNOWN
}

fn check_version(version: u32) -> HRESULT {
    match version {
        0x800 => S_OK,
        v if v < 0x800 => DIERR_OLDDIRECTINPUTVERSION,
        _ => DIERR_BETADIRECTINPUTVERSION,
    }
}

fn matches_gamepad_filter(dev_type: u32) -> bool {
    matches!(
        dev_type & 0xFF,
        DI8DEVCLASS_ALL | DI8DEVCLASS_GAMECTRL | DI8DEVTYPE_JOYSTICK | DI8DEVTYPE_GAMEPAD | DI8DEVTYPE_1STPERSON
    )
}

unsafe extern "win64" fn query_interface(this: *mut Obj, iid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() || iid.is_null() {
        return E_POINTER;
    }
    let g = iid.read_unaligned();
    if g == IID_IUNKNOWN || (g == IID_IDIRECTINPUT8W && (*this).wide) || (g == IID_IDIRECTINPUT8A && !(*this).wide) {
        add_ref(this);
        out.write_unaligned(this as *mut c_void);
        S_OK
    } else if g == IID_IDIRECTINPUT8W || g == IID_IDIRECTINPUT8A {
        out.write_unaligned(new_obj(g == IID_IDIRECTINPUT8W));
        S_OK
    } else {
        out.write_unaligned(std::ptr::null_mut());
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn add_ref(this: *mut Obj) -> u32 {
    (*this).refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn release(this: *mut Obj) -> u32 {
    let n = (*this).refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if n == 0 {
        drop(Box::from_raw(this));
    }
    n
}

unsafe extern "win64" fn create_device(this: *mut Obj, g: *const Guid, out: *mut *mut c_void, outer: *mut c_void) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    out.write_unaligned(std::ptr::null_mut());
    if !outer.is_null() {
        return CLASS_E_NOAGGREGATION;
    }
    if g.is_null() {
        return DIERR_INVALIDPARAM;
    }
    ensure_gamepad_started();
    let want = g.read_unaligned();
    if let Some(slot) = slot_from_guid(&want) {
        if xinput1_4::slot_state(slot).is_some() {
            let wide = (*this).wide;
            out.write_unaligned(new_device(slot, wide));
            log::info!("IDirectInput8::CreateDevice(slot {slot}, wide={wide}) -> S_OK");
            return S_OK;
        }
    }
    log::info!("IDirectInput8::CreateDevice -> DIERR_DEVICENOTREG");
    DIERR_DEVICENOTREG
}

unsafe extern "win64" fn enum_devices(this: *mut Obj, dev_type: u32, cb: usize, pv_ref: *mut c_void, flags: u32) -> HRESULT {
    if cb == 0 {
        return DIERR_INVALIDPARAM;
    }
    ensure_gamepad_started();
    // XInput / GameController pads do not expose DirectInput force feedback
    // (preserves Wwise AkDirectInputRumbleSink fallback to XInput).
    if flags & DIEDFL_FORCEFEEDBACK != 0 || !matches_gamepad_filter(dev_type) {
        log::info!("IDirectInput8::EnumDevices(type {dev_type:#x}, flags {flags:#x}) -> 0 matching devices");
        return S_OK;
    }
    let wide = (*this).wide;
    let mut found = 0usize;
    for slot in 0..XUSER_MAX_COUNT {
        if xinput1_4::slot_state(slot).is_none() {
            continue;
        }
        found += 1;
        let cont = if wide {
            let mut inst = make_device_instance_w(slot);
            crate::call_guest8(cb, (&mut inst as *mut DIDEVICEINSTANCEW) as u64, pv_ref as u64, 0, 0, 0, 0, 0, 0) as i32
        } else {
            let mut inst = make_device_instance_a(slot);
            crate::call_guest8(cb, (&mut inst as *mut DIDEVICEINSTANCEA) as u64, pv_ref as u64, 0, 0, 0, 0, 0, 0) as i32
        };
        if cont == DIENUM_STOP {
            break;
        }
    }
    log::info!("IDirectInput8::EnumDevices(type {dev_type:#x}, flags {flags:#x}, wide={wide}) -> {found} pad(s)");
    S_OK
}

unsafe extern "win64" fn get_device_status(_this: *mut Obj, g: *const Guid) -> HRESULT {
    if g.is_null() {
        return DIERR_INVALIDPARAM;
    }
    ensure_gamepad_started();
    if let Some(slot) = slot_from_guid(&g.read_unaligned()) {
        if xinput1_4::slot_state(slot).is_some() {
            return DI_OK;
        }
    }
    DI_NOTATTACHED
}

/// Native-unsupported on macOS/iOS: launches Windows `joy.cpl` Game Controllers Control Panel.
/// Matches Microsoft Learn `IDirectInput8::RunControlPanel`: returns `DIERR_INVALIDPARAM`
/// when reserved `dwFlags != 0` and `DIERR_UNSUPPORTED` when the Control Panel applet is absent.
unsafe extern "win64" fn run_control_panel(_this: *mut Obj, _hwnd: usize, flags: u32) -> HRESULT {
    if flags != 0 {
        return DIERR_INVALIDPARAM;
    }
    DIERR_UNSUPPORTED
}

unsafe extern "win64" fn initialize(_this: *mut Obj, _hinst: usize, version: u32) -> HRESULT {
    let hr = check_version(version);
    if hr == S_OK {
        ensure_gamepad_started();
    }
    hr
}

unsafe extern "win64" fn find_device(this: *mut Obj, _cls: *const Guid, name: *const u16, out: *mut Guid) -> HRESULT {
    if name.is_null() || out.is_null() {
        return DIERR_INVALIDPARAM;
    }
    ensure_gamepad_started();
    let target = if (*this).wide {
        crate::strings::from_wide(name)
    } else {
        crate::strings::from_ansi(name as *const u8)
    };
    for slot in 0..XUSER_MAX_COUNT {
        if xinput1_4::slot_state(slot).is_some() && slot_instance_name(slot).eq_ignore_ascii_case(&target) {
            out.write_unaligned(slot_guid(slot));
            return DI_OK;
        }
    }
    DIERR_DEVICENOTREG
}

unsafe extern "win64" fn enum_devices_by_semantics(_this: *mut Obj, _user: *const u16, _fmt: *mut c_void, cb: usize, _r: *mut c_void, _f: u32) -> HRESULT {
    if cb == 0 {
        return DIERR_INVALIDPARAM;
    }
    log::info!("IDirectInput8::EnumDevicesBySemantics -> no devices");
    S_OK
}

/// Native-unsupported (matches Windows Vista+): Microsoft Learn `IDirectInput8::ConfigureDevices`
/// explicitly documents that this method was deprecated and removed on Windows Vista and later,
/// returning `DIERR_UNSUPPORTED`.
unsafe extern "win64" fn configure_devices(_this: *mut Obj, _cb: usize, _p: *mut c_void, _f: u32, _r: *mut c_void) -> HRESULT {
    DIERR_UNSUPPORTED
}

/// `CoCreateInstance` hook (called by ole32). `None` when `clsid` is not
/// DirectInput8.
pub unsafe fn create_instance(clsid: *const u8, iid: *const u8, outer: *mut c_void, out: *mut *mut c_void) -> Option<HRESULT> {
    if clsid.is_null() || (clsid as *const Guid).read_unaligned() != CLSID_DIRECTINPUT8 {
        return None;
    }
    if out.is_null() {
        return Some(E_POINTER);
    }
    if !outer.is_null() {
        return Some(CLASS_E_NOAGGREGATION);
    }
    if iid.is_null() {
        return Some(E_NOINTERFACE);
    }
    let g = (iid as *const Guid).read_unaligned();
    if !is_di8(&g) {
        return Some(E_NOINTERFACE);
    }
    let wide = g != IID_IDIRECTINPUT8A;
    log::info!("CoCreateInstance(CLSID_DirectInput8, wide={wide}): IDirectInput8 ready");
    out.write_unaligned(new_obj(wide));
    Some(S_OK)
}

pub unsafe extern "win64" fn DirectInput8Create(_hinst: usize, version: u32, iid: *const Guid, out: *mut *mut c_void, outer: *mut c_void) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    out.write_unaligned(std::ptr::null_mut());
    if !outer.is_null() {
        return CLASS_E_NOAGGREGATION;
    }
    if iid.is_null() {
        return E_NOINTERFACE;
    }
    let g = iid.read_unaligned();
    if !is_di8(&g) {
        return E_NOINTERFACE;
    }
    let hr = check_version(version);
    if hr != S_OK {
        return hr;
    }
    ensure_gamepad_started();
    out.write_unaligned(new_obj(g != IID_IDIRECTINPUT8A));
    S_OK
}

#[cfg(test)]
mod tests {
    use super::*;
    use xinput1_4::{backend_connect, backend_disconnect, backend_update, DeviceInfo, GcPad};

    static SERIAL: Mutex<()> = Mutex::new(());

    unsafe extern "win64" fn never(_d: *const c_void, _r: *mut c_void) -> i32 {
        panic!("no device expected");
    }

    #[test]
    fn wwise_rumble_sink_sequence() {
        let _g = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            // Even with a connected pad, DIEDFL_FORCEFEEDBACK (0x100) must report 0 devices.
            backend_connect(0, DeviceInfo { name: "Xbox".into(), wireless: false, rumble: true, battery: None });
            let mut p: *mut c_void = std::ptr::null_mut();
            let hr = create_instance(CLSID_DIRECTINPUT8.as_ptr(), IID_IDIRECTINPUT8W.as_ptr(), std::ptr::null_mut(), &mut p);
            assert_eq!(hr, Some(S_OK));
            let o = p as *mut Obj;
            let vt = &*(*o).vtbl;
            assert_eq!((vt.initialize)(o, 0x1_4000_0000, 0x800), S_OK);
            assert_eq!((vt.enum_devices)(o, DI8DEVCLASS_GAMECTRL, never as *const () as usize, std::ptr::null_mut(), 0x101), S_OK);
            let mut d: *mut c_void = 1 as *mut c_void;
            assert_eq!((vt.create_device)(o, &IID_IUNKNOWN, &mut d, std::ptr::null_mut()), DIERR_DEVICENOTREG);
            assert!(d.is_null());
            assert_eq!((vt.add_ref)(o), 2);
            assert_eq!((vt.release)(o), 1);
            assert_eq!((vt.release)(o), 0);
            let other = guid(1, 2, 3, [0; 8]);
            assert_eq!(create_instance(other.as_ptr(), IID_IDIRECTINPUT8W.as_ptr(), std::ptr::null_mut(), &mut p), None);
            assert_eq!(DirectInput8Create(0, 0x700, &IID_IDIRECTINPUT8W, &mut p, std::ptr::null_mut()), DIERR_OLDDIRECTINPUTVERSION);
            backend_disconnect(0);
        }
    }

    unsafe extern "win64" fn collect_w(inst: *const DIDEVICEINSTANCEW, ctx: *mut c_void) -> i32 {
        let out = &mut *(ctx as *mut Vec<DIDEVICEINSTANCEW>);
        out.push(inst.read_unaligned());
        1 // DIENUM_CONTINUE
    }

    unsafe extern "win64" fn collect_a_stop_first(inst: *const DIDEVICEINSTANCEA, ctx: *mut c_void) -> i32 {
        let out = &mut *(ctx as *mut Vec<DIDEVICEINSTANCEA>);
        out.push(inst.read_unaligned());
        DIENUM_STOP
    }

    unsafe extern "win64" fn collect_obj_w(inst: *const DIDEVICEOBJECTINSTANCEW, ctx: *mut c_void) -> i32 {
        let out = &mut *(ctx as *mut Vec<(u32, u32, String)>);
        let o = inst.read_unaligned();
        out.push((o.dw_ofs, o.dw_type, crate::strings::from_wide(o.tsz_name.as_ptr())));
        1
    }

    #[test]
    fn gamepad_enum_create_properties_and_polling() {
        let _g = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            // Ensure clean slots.
            for s in 0..XUSER_MAX_COUNT {
                backend_disconnect(s);
            }
            backend_connect(1, DeviceInfo { name: "Test Pad 1".into(), wireless: false, rumble: true, battery: None });
            backend_connect(2, DeviceInfo { name: "Test Pad 2".into(), wireless: true, rumble: true, battery: Some(0.8) });

            // Create IDirectInput8W and enumerate gamepads.
            let mut di_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(DirectInput8Create(0, 0x800, &IID_IDIRECTINPUT8W, &mut di_ptr, std::ptr::null_mut()), S_OK);
            let di = di_ptr as *mut Obj;
            let vt = &*(*di).vtbl;

            let mut found_w: Vec<DIDEVICEINSTANCEW> = Vec::new();
            assert_eq!(
                (vt.enum_devices)(di, DI8DEVCLASS_GAMECTRL, collect_w as *const () as usize, &mut found_w as *mut _ as *mut c_void, 1),
                S_OK
            );
            assert_eq!(found_w.len(), 2);
            assert_eq!(found_w[0].dw_size, 1100);
            assert_eq!(found_w[0].guid_instance, slot_guid(1));
            assert_eq!(found_w[0].guid_product, GAMEPAD_PRODUCT_GUID);
            assert_eq!(found_w[0].dw_dev_type, GAMEPAD_DEVTYPE);
            assert_eq!(crate::strings::from_wide(found_w[0].tsz_instance_name.as_ptr()), "Controller (Gamepad 2)");
            assert_eq!(found_w[1].guid_instance, slot_guid(2));
            assert_eq!((vt.get_device_status)(di, &slot_guid(1)), DI_OK);
            assert_eq!((vt.get_device_status)(di, &slot_guid(0)), DI_NOTATTACHED);

            // Create IDirectInput8A and verify DIENUM_STOP stops after first device.
            let mut dia_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(DirectInput8Create(0, 0x800, &IID_IDIRECTINPUT8A, &mut dia_ptr, std::ptr::null_mut()), S_OK);
            let dia = dia_ptr as *mut Obj;
            let vta = &*(*dia).vtbl;
            let mut found_a: Vec<DIDEVICEINSTANCEA> = Vec::new();
            assert_eq!(
                (vta.enum_devices)(dia, DI8DEVCLASS_ALL, collect_a_stop_first as *const () as usize, &mut found_a as *mut _ as *mut c_void, 1),
                S_OK
            );
            assert_eq!(found_a.len(), 1);
            assert_eq!(found_a[0].dw_size, 580);
            assert_eq!(crate::strings::from_ansi(found_a[0].tsz_instance_name.as_ptr()), "Controller (Gamepad 2)");
            assert_eq!((vta.release)(dia), 0);

            // Create device for slot 1.
            let mut dev_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!((vt.create_device)(di, &found_w[0].guid_instance, &mut dev_ptr, std::ptr::null_mut()), S_OK);
            assert!(!dev_ptr.is_null());
            let dev = dev_ptr as *mut DeviceObj;
            let dvt = &*(*dev).vtbl;

            // Capabilities & GetDeviceInfo & EnumObjects.
            let mut caps = DIDEVCAPS { dw_size: std::mem::size_of::<DIDEVCAPS>() as u32, ..Default::default() };
            assert_eq!((dvt.get_capabilities)(dev, &mut caps), DI_OK);
            assert_eq!(caps.dw_flags, DIDC_ATTACHED);
            assert_eq!(caps.dw_dev_type, GAMEPAD_DEVTYPE);
            assert_eq!((caps.dw_axes, caps.dw_buttons, caps.dw_povs), (7, 12, 1));

            let mut dev_info = std::mem::zeroed::<DIDEVICEINSTANCEW>();
            dev_info.dw_size = std::mem::size_of::<DIDEVICEINSTANCEW>() as u32;
            assert_eq!((dvt.get_device_info)(dev, &mut dev_info as *mut _ as *mut u8), DI_OK);
            assert_eq!(dev_info.guid_instance, slot_guid(1));

            let mut objs: Vec<(u32, u32, String)> = Vec::new();
            assert_eq!((dvt.enum_objects)(dev, collect_obj_w as *const () as usize, &mut objs as *mut _ as *mut c_void, DIDFT_ALL), DI_OK);
            assert_eq!(objs.len(), 7 + 1 + 12);

            // Properties: VIDPID, INSTANCENAME, PRODUCTNAME, default DIPROP_RANGE.
            let mut vidpid = DIPROPDWORD {
                diph: DIPROPHEADER { dw_size: 20, dw_header_size: 16, dw_obj: 0, dw_how: DIPH_DEVICE },
                dw_data: 0,
            };
            assert_eq!((dvt.get_property)(dev, DIPROP_VIDPID as *const Guid, &mut vidpid.diph), DI_OK);
            assert_eq!(vidpid.dw_data, GAMEPAD_VIDPID);

            let mut pstr = DIPROPSTRING {
                diph: DIPROPHEADER { dw_size: 536, dw_header_size: 16, dw_obj: 0, dw_how: DIPH_DEVICE },
                wsz: [0; MAX_PATH],
            };
            assert_eq!((dvt.get_property)(dev, DIPROP_INSTANCENAME as *const Guid, &mut pstr.diph), DI_OK);
            assert_eq!(crate::strings::from_wide(pstr.wsz.as_ptr()), "Controller (Gamepad 2)");
            assert_eq!((dvt.get_property)(dev, DIPROP_PRODUCTNAME as *const Guid, &mut pstr.diph), DI_OK);
            assert_eq!(crate::strings::from_wide(pstr.wsz.as_ptr()), PRODUCT_NAME);

            let mut rng = DIPROPRANGE {
                diph: DIPROPHEADER { dw_size: 24, dw_header_size: 16, dw_obj: 0, dw_how: DIPH_DEVICE },
                l_min: 0,
                l_max: 0,
            };
            assert_eq!((dvt.get_property)(dev, DIPROP_RANGE as *const Guid, &mut rng.diph), DI_OK);
            assert_eq!((rng.l_min, rng.l_max), (0, 65535));

            // SetDataFormat for DIJOYSTATE (80 bytes).
            let df_joy = DIDATAFORMAT {
                dw_size: 32,
                dw_obj_size: 24,
                dw_flags: 1,
                dw_data_size: 80,
                dw_num_objs: 0,
                rgodf: std::ptr::null(),
            };
            assert_eq!((dvt.set_data_format)(dev, &df_joy), DI_OK);
            assert_eq!((dvt.set_cooperative_level)(dev, 0, DISCL_NONEXCLUSIVE | DISCL_BACKGROUND), DI_OK);

            // Must be acquired before Poll / GetDeviceState.
            let mut js = std::mem::zeroed::<DIJOYSTATE>();
            assert_eq!((dvt.poll)(dev), DIERR_NOTACQUIRED);
            assert_eq!((dvt.get_device_state)(dev, 80, &mut js as *mut _ as *mut c_void), DIERR_NOTACQUIRED);
            assert_eq!((dvt.acquire)(dev), DI_OK);
            assert_eq!((dvt.acquire)(dev), DI_NOEFFECT);

            // Drive synthetic gamepad state and read DIJOYSTATE at default range 0..65535.
            let pad = GcPad {
                a: true,
                y: true,
                right_shoulder: true,
                menu: true,
                dpad_up: true,
                dpad_right: true,
                lx: -1.0,
                ly: 1.0, // +Y up -> lMin (0) in DirectInput
                rx: 1.0,
                ry: -1.0, // -Y down -> lMax (65535) in DirectInput
                left_trigger: 1.0,
                right_trigger: 0.0,
                ..Default::default()
            }
            .to_xinput();
            backend_update(1, pad);

            assert_eq!((dvt.poll)(dev), DI_OK);
            assert_eq!((dvt.get_device_state)(dev, 80, &mut js as *mut _ as *mut c_void), DI_OK);
            assert_eq!(js.l_x, 0);
            assert_eq!(js.l_y, 0);
            assert_eq!(js.l_z, 65535);
            assert_eq!(js.l_rx, 65535);
            assert_eq!(js.l_ry, 65535);
            assert_eq!(js.rgl_slider, [65535, 0]);
            assert_eq!(js.rgdw_pov[0], 4500); // NE
            assert_eq!(js.rgdw_pov[1], u32::MAX);
            assert_eq!(js.rgb_buttons[0], 0x80); // A
            assert_eq!(js.rgb_buttons[1], 0x00); // B
            assert_eq!(js.rgb_buttons[3], 0x80); // Y
            assert_eq!(js.rgb_buttons[5], 0x80); // RB
            assert_eq!(js.rgb_buttons[7], 0x80); // Start

            // Reconfigure DIPROP_RANGE to -32768..32767 and test DIJOYSTATE2 (272 bytes) with explicit rgodf!
            let set_rng = DIPROPRANGE {
                diph: DIPROPHEADER { dw_size: 24, dw_header_size: 16, dw_obj: 0, dw_how: DIPH_DEVICE },
                l_min: -32768,
                l_max: 32767,
            };
            assert_eq!((dvt.set_property)(dev, DIPROP_RANGE as *const Guid, &set_rng.diph), DI_OK);
            let mut read_rng = set_rng;
            read_rng.l_min = 0;
            read_rng.l_max = 0;
            assert_eq!((dvt.get_property)(dev, DIPROP_RANGE as *const Guid, &mut read_rng.diph), DI_OK);
            assert_eq!((read_rng.l_min, read_rng.l_max), (-32768, 32767));

            let odf = [
                DIOBJECTDATAFORMAT { pguid: &GUID_XAXIS, dw_ofs: 0, dw_type: DIDFT_ABSAXIS | DIDFT_ANYINSTANCE, dw_flags: 0 },
                DIOBJECTDATAFORMAT { pguid: &GUID_YAXIS, dw_ofs: 4, dw_type: DIDFT_ABSAXIS | DIDFT_ANYINSTANCE, dw_flags: 0 },
                DIOBJECTDATAFORMAT { pguid: &GUID_ZAXIS, dw_ofs: 8, dw_type: DIDFT_ABSAXIS | DIDFT_ANYINSTANCE, dw_flags: 0 },
                DIOBJECTDATAFORMAT { pguid: &GUID_RXAXIS, dw_ofs: 12, dw_type: DIDFT_ABSAXIS | DIDFT_ANYINSTANCE, dw_flags: 0 },
                DIOBJECTDATAFORMAT { pguid: &GUID_RYAXIS, dw_ofs: 16, dw_type: DIDFT_ABSAXIS | DIDFT_ANYINSTANCE, dw_flags: 0 },
                DIOBJECTDATAFORMAT { pguid: &GUID_SLIDER, dw_ofs: 24, dw_type: DIDFT_ABSAXIS | DIDFT_ANYINSTANCE, dw_flags: 0 },
                DIOBJECTDATAFORMAT { pguid: &GUID_SLIDER, dw_ofs: 28, dw_type: DIDFT_ABSAXIS | DIDFT_ANYINSTANCE, dw_flags: 0 },
                DIOBJECTDATAFORMAT { pguid: &GUID_POV, dw_ofs: 32, dw_type: DIDFT_POV | DIDFT_ANYINSTANCE, dw_flags: 0 },
                DIOBJECTDATAFORMAT { pguid: &GUID_BUTTON, dw_ofs: 48, dw_type: DIDFT_BUTTON | DIDFT_ANYINSTANCE, dw_flags: 0 },
                DIOBJECTDATAFORMAT { pguid: &GUID_BUTTON, dw_ofs: 49, dw_type: DIDFT_BUTTON | DIDFT_ANYINSTANCE, dw_flags: 0 },
                DIOBJECTDATAFORMAT { pguid: &GUID_BUTTON, dw_ofs: 50, dw_type: DIDFT_BUTTON | DIDFT_ANYINSTANCE, dw_flags: 0 },
            ];
            let df_joy2 = DIDATAFORMAT {
                dw_size: 32,
                dw_obj_size: 24,
                dw_flags: 1,
                dw_data_size: 272,
                dw_num_objs: odf.len() as u32,
                rgodf: odf.as_ptr(),
            };
            assert_eq!((dvt.set_data_format)(dev, &df_joy2), DI_OK);

            // Set centered sticks, RT full, B pressed, D-pad West (27000).
            let pad2 = GcPad {
                b: true,
                dpad_left: true,
                lx: 0.0,
                ly: 0.0,
                rx: 0.5,
                ry: 0.5,
                left_trigger: 0.0,
                right_trigger: 1.0,
                ..Default::default()
            }
            .to_xinput();
            backend_update(1, pad2);

            let mut js2 = std::mem::zeroed::<DIJOYSTATE2>();
            assert_eq!((dvt.get_device_state)(dev, 272, &mut js2 as *mut _ as *mut c_void), DI_OK);
            assert_eq!(js2.l_x, 0);
            assert_eq!(js2.l_y, 0);
            assert_eq!(js2.l_z, -32768); // RT pulls Z toward lMin
            assert!((js2.l_rx - 16384).abs() <= 1);
            assert!((js2.l_ry + 16384).abs() <= 1); // +Y up -> negative lRy
            assert_eq!(js2.rgl_slider, [-32768, 32767]);
            assert_eq!(js2.rgdw_pov[0], 27000);
            assert_eq!(js2.rgb_buttons[0], 0x00);
            assert_eq!(js2.rgb_buttons[1], 0x80); // B

            // Buffered events via DIPROP_BUFFERSIZE + GetDeviceData.
            let buf_prop = DIPROPDWORD {
                diph: DIPROPHEADER { dw_size: 20, dw_header_size: 16, dw_obj: 0, dw_how: DIPH_DEVICE },
                dw_data: 16,
            };
            assert_eq!((dvt.set_property)(dev, DIPROP_BUFFERSIZE as *const Guid, &buf_prop.diph), DI_OK);
            let mut pad3 = pad2;
            pad3.buttons |= btn::X;
            backend_update(1, pad3);
            let mut dod = [DIDEVICEOBJECTDATA::default(); 8];
            let mut n = 8u32;
            assert_eq!((dvt.get_device_data)(dev, 24, dod.as_mut_ptr() as *mut c_void, &mut n, 0), DI_OK);
            assert_eq!(n, 1);
            assert_eq!((dod[0].dw_ofs, dod[0].dw_data), (50, 0x80)); // button 2 (X)

            // Disconnecting causes DIERR_INPUTLOST cleanly.
            backend_disconnect(1);
            backend_disconnect(2);
            assert_eq!((dvt.poll)(dev), DIERR_INPUTLOST);
            assert_eq!((dvt.get_device_state)(dev, 272, &mut js2 as *mut _ as *mut c_void), DIERR_NOTACQUIRED);

            assert_eq!((dvt.unacquire)(dev), DI_NOEFFECT);
            assert_eq!((dvt.release)(dev), 0);
            assert_eq!((vt.release)(di), 0);
        }
    }

    #[test]
    fn deadzone_pov_and_find_device() {
        let _g = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            for s in 0..XUSER_MAX_COUNT {
                backend_disconnect(s);
            }
            // Verify all 8 POV directions + opposing-direction cancellation.
            assert_eq!(dpad_pov(0), u32::MAX);
            assert_eq!(dpad_pov(btn::DPAD_UP), 0);
            assert_eq!(dpad_pov(btn::DPAD_UP | btn::DPAD_RIGHT), 4500);
            assert_eq!(dpad_pov(btn::DPAD_RIGHT), 9000);
            assert_eq!(dpad_pov(btn::DPAD_DOWN | btn::DPAD_RIGHT), 13500);
            assert_eq!(dpad_pov(btn::DPAD_DOWN), 18000);
            assert_eq!(dpad_pov(btn::DPAD_DOWN | btn::DPAD_LEFT), 22500);
            assert_eq!(dpad_pov(btn::DPAD_LEFT), 27000);
            assert_eq!(dpad_pov(btn::DPAD_UP | btn::DPAD_LEFT), 31500);
            assert_eq!(dpad_pov(btn::DPAD_UP | btn::DPAD_DOWN), u32::MAX);

            backend_connect(0, DeviceInfo { name: "Pad0".into(), wireless: false, rumble: false, battery: None });

            let mut di_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(DirectInput8Create(0, 0x800, &IID_IDIRECTINPUT8W, &mut di_ptr, std::ptr::null_mut()), S_OK);
            let di = di_ptr as *mut Obj;
            let vt = &*(*di).vtbl;

            // FindDevice by instance name.
            let name_w = crate::strings::to_wide_z("Controller (Gamepad 1)");
            let mut found_guid: Guid = [0; 16];
            assert_eq!((vt.find_device)(di, &GUID_XAXIS, name_w.as_ptr(), &mut found_guid), DI_OK);
            assert_eq!(found_guid, slot_guid(0));

            let mut dev_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!((vt.create_device)(di, &found_guid, &mut dev_ptr, std::ptr::null_mut()), S_OK);
            let dev = dev_ptr as *mut DeviceObj;
            let dvt = &*(*dev).vtbl;

            // SetDataFormat for DIJOYSTATE + set range -1000..1000 + set 25% deadzone on lX only.
            let df = DIDATAFORMAT { dw_size: 32, dw_obj_size: 24, dw_flags: 1, dw_data_size: 80, dw_num_objs: 0, rgodf: std::ptr::null() };
            assert_eq!((dvt.set_data_format)(dev, &df), DI_OK);
            let rng = DIPROPRANGE { diph: DIPROPHEADER { dw_size: 24, dw_header_size: 16, dw_obj: 0, dw_how: DIPH_DEVICE }, l_min: -1000, l_max: 1000 };
            assert_eq!((dvt.set_property)(dev, DIPROP_RANGE as *const Guid, &rng.diph), DI_OK);
            let dz = DIPROPDWORD { diph: DIPROPHEADER { dw_size: 20, dw_header_size: 16, dw_obj: 0, dw_how: DIPH_BYOFFSET }, dw_data: 2500 };
            assert_eq!((dvt.set_property)(dev, DIPROP_DEADZONE as *const Guid, &dz.diph), DI_OK);

            // Axis mode: ABS supported, REL unsupported.
            let mut mode = DIPROPDWORD { diph: DIPROPHEADER { dw_size: 20, dw_header_size: 16, dw_obj: 0, dw_how: DIPH_DEVICE }, dw_data: 99 };
            assert_eq!((dvt.get_property)(dev, DIPROP_AXISMODE as *const Guid, &mut mode.diph), DI_OK);
            assert_eq!(mode.dw_data, DIPROPAXISMODE_ABS);
            mode.dw_data = 1;
            assert_eq!((dvt.set_property)(dev, DIPROP_AXISMODE as *const Guid, &mode.diph), DIERR_UNSUPPORTED);

            // GetObjectInfo by offset 0 -> X Axis.
            let mut oi = std::mem::zeroed::<DIDEVICEOBJECTINSTANCEW>();
            oi.dw_size = std::mem::size_of::<DIDEVICEOBJECTINSTANCEW>() as u32;
            assert_eq!((dvt.get_object_info)(dev, &mut oi as *mut _ as *mut u8, 0, DIPH_BYOFFSET), DI_OK);
            assert_eq!(oi.guid_type, GUID_XAXIS);
            assert_eq!(crate::strings::from_wide(oi.tsz_name.as_ptr()), "X Axis");

            assert_eq!((dvt.acquire)(dev), DI_OK);
            // lx = 0.2 (inside 25% deadzone -> 0), ly = -0.2 (no deadzone on lY -> +200).
            backend_update(0, GcPad { lx: 0.2, ly: -0.2, ..Default::default() }.to_xinput());
            let mut js = std::mem::zeroed::<DIJOYSTATE>();
            assert_eq!((dvt.get_device_state)(dev, 80, &mut js as *mut _ as *mut c_void), DI_OK);
            assert_eq!(js.l_x, 0);
            assert!((js.l_y - 200).abs() <= 2);

            // Extended GetProperty / SetProperty coverage (GUIDANDPATH, APPDATA, FFGAIN, AUTOCENTER,
            // CALIBRATIONMODE, CALIBRATION, KEYNAME, TYPENAME, GETPORTDISPLAYNAME, USERNAME).
            let mut gp = DIPROPGUIDANDPATH {
                diph: DIPROPHEADER { dw_size: 552, dw_header_size: 16, dw_obj: 0, dw_how: DIPH_DEVICE },
                guid_class: [0; 16],
                wsz_path: [0; MAX_PATH],
            };
            assert_eq!((dvt.get_property)(dev, DIPROP_GUIDANDPATH as *const Guid, &mut gp.diph), DI_OK);
            assert_eq!(gp.guid_class, crate::misc::hid::GUID_DEVINTERFACE_HID);
            assert!(crate::strings::from_wide(gp.wsz_path.as_ptr()).contains("ig_00"));

            let mut app = DIPROPPOINTER {
                diph: DIPROPHEADER { dw_size: 24, dw_header_size: 16, dw_obj: 0, dw_how: DIPH_BYOFFSET },
                u_data: 0xDEAD_BEEF,
            };
            assert_eq!((dvt.set_property)(dev, DIPROP_APPDATA as *const Guid, &app.diph), DI_OK);
            app.u_data = 0;
            assert_eq!((dvt.get_property)(dev, DIPROP_APPDATA as *const Guid, &mut app.diph), DI_OK);
            assert_eq!(app.u_data, 0xDEAD_BEEF);

            let mut cal_mode = DIPROPDWORD {
                diph: DIPROPHEADER { dw_size: 20, dw_header_size: 16, dw_obj: 0, dw_how: DIPH_DEVICE },
                dw_data: DIPROPCALIBRATIONMODE_RAW,
            };
            assert_eq!((dvt.set_property)(dev, DIPROP_CALIBRATIONMODE as *const Guid, &cal_mode.diph), DI_OK);
            cal_mode.dw_data = 0;
            assert_eq!((dvt.get_property)(dev, DIPROP_CALIBRATIONMODE as *const Guid, &mut cal_mode.diph), DI_OK);
            assert_eq!(cal_mode.dw_data, DIPROPCALIBRATIONMODE_RAW);

            let mut kn = DIPROPSTRING {
                diph: DIPROPHEADER { dw_size: 536, dw_header_size: 16, dw_obj: 0, dw_how: DIPH_BYOFFSET },
                wsz: [0; MAX_PATH],
            };
            assert_eq!((dvt.get_property)(dev, DIPROP_KEYNAME as *const Guid, &mut kn.diph), DI_OK);
            assert_eq!(crate::strings::from_wide(kn.wsz.as_ptr()), "X Axis");

            // Native-unsupported Control Panel & ConfigureDevices match Windows semantics.
            assert_eq!((dvt.run_control_panel)(dev, 0, 1), DIERR_INVALIDPARAM);
            assert_eq!((dvt.run_control_panel)(dev, 0, 0), DIERR_UNSUPPORTED);
            assert_eq!((vt.run_control_panel)(di, 0, 1), DIERR_INVALIDPARAM);
            assert_eq!((vt.run_control_panel)(di, 0, 0), DIERR_UNSUPPORTED);
            assert_eq!((vt.configure_devices)(di, 0, std::ptr::null_mut(), 0, std::ptr::null_mut()), DIERR_UNSUPPORTED);

            backend_disconnect(0);
            assert_eq!((dvt.release)(dev), 0);
            assert_eq!((vt.release)(di), 0);
        }
    }

    #[test]
    fn raw_calibration_mode_bypasses_range_deadzone_saturation_and_calibration() {
        let _g = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            for s in 0..XUSER_MAX_COUNT {
                backend_disconnect(s);
            }
            backend_connect(0, DeviceInfo { name: "CalPad".into(), wireless: false, rumble: false, battery: None });

            let mut di_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(DirectInput8Create(0, 0x800, &IID_IDIRECTINPUT8W, &mut di_ptr, std::ptr::null_mut()), S_OK);
            let di = di_ptr as *mut Obj;
            let vt = &*(*di).vtbl;

            let mut dev_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!((vt.create_device)(di, &slot_guid(0), &mut dev_ptr, std::ptr::null_mut()), S_OK);
            let dev = dev_ptr as *mut DeviceObj;
            let dvt = &*(*dev).vtbl;

            let df = DIDATAFORMAT { dw_size: 32, dw_obj_size: 24, dw_flags: 1, dw_data_size: 80, dw_num_objs: 0, rgodf: std::ptr::null() };
            assert_eq!((dvt.set_data_format)(dev, &df), DI_OK);

            // Configure cooked range (-1000..1000), 25% deadzone, 80% saturation, and buffered events.
            let rng = DIPROPRANGE { diph: DIPROPHEADER { dw_size: 24, dw_header_size: 16, dw_obj: 0, dw_how: DIPH_DEVICE }, l_min: -1000, l_max: 1000 };
            assert_eq!((dvt.set_property)(dev, DIPROP_RANGE as *const Guid, &rng.diph), DI_OK);
            let dz = DIPROPDWORD { diph: DIPROPHEADER { dw_size: 20, dw_header_size: 16, dw_obj: 0, dw_how: DIPH_BYOFFSET }, dw_data: 2500 };
            assert_eq!((dvt.set_property)(dev, DIPROP_DEADZONE as *const Guid, &dz.diph), DI_OK);
            let sat = DIPROPDWORD { diph: DIPROPHEADER { dw_size: 20, dw_header_size: 16, dw_obj: 0, dw_how: DIPH_BYOFFSET }, dw_data: 8000 };
            assert_eq!((dvt.set_property)(dev, DIPROP_SATURATION as *const Guid, &sat.diph), DI_OK);
            let buf_prop = DIPROPDWORD { diph: DIPROPHEADER { dw_size: 20, dw_header_size: 16, dw_obj: 0, dw_how: DIPH_DEVICE }, dw_data: 16 };
            assert_eq!((dvt.set_property)(dev, DIPROP_BUFFERSIZE as *const Guid, &buf_prop.diph), DI_OK);

            backend_update(0, GcPad::default().to_xinput());
            assert_eq!((dvt.acquire)(dev), DI_OK);

            // COOKED mode: moving lx from 0.0 -> 0.2 stays inside 25% deadzone (immediate=0, 0 buffered events).
            backend_update(0, GcPad { lx: 0.2, ..Default::default() }.to_xinput());
            let mut js = std::mem::zeroed::<DIJOYSTATE>();
            assert_eq!((dvt.get_device_state)(dev, 80, &mut js as *mut _ as *mut c_void), DI_OK);
            assert_eq!(js.l_x, 0);
            let mut dod = [DIDEVICEOBJECTDATA::default(); 8];
            let mut n = 8u32;
            assert_eq!((dvt.get_device_data)(dev, 24, dod.as_mut_ptr() as *mut c_void, &mut n, 0), DI_OK);
            assert_eq!(n, 0);

            // COOKED mode: lx = 0.85 exceeds 80% saturation -> clamps to l_max (1000).
            backend_update(0, GcPad { lx: 0.85, ..Default::default() }.to_xinput());
            assert_eq!((dvt.get_device_state)(dev, 80, &mut js as *mut _ as *mut c_void), DI_OK);
            assert_eq!(js.l_x, 1000);
            n = 8;
            assert_eq!((dvt.get_device_data)(dev, 24, dod.as_mut_ptr() as *mut c_void, &mut n, 0), DI_OK);
            assert_eq!(n, 1);
            assert_eq!((dod[0].dw_ofs, dod[0].dw_data as i32), (0, 1000));

            // Switch to DIPROPCALIBRATIONMODE_RAW: range, deadzone, saturation, and calibration are bypassed.
            let raw_mode = DIPROPDWORD { diph: DIPROPHEADER { dw_size: 20, dw_header_size: 16, dw_obj: 0, dw_how: DIPH_DEVICE }, dw_data: DIPROPCALIBRATIONMODE_RAW };
            assert_eq!((dvt.set_property)(dev, DIPROP_CALIBRATIONMODE as *const Guid, &raw_mode.diph), DI_OK);

            // Immediate GetDeviceState at lx = 0.85 must report raw physical (~60620), neither 1000 nor saturated 65535.
            assert_eq!((dvt.get_device_state)(dev, 80, &mut js as *mut _ as *mut c_void), DI_OK);
            assert!((js.l_x - 60_620).abs() <= 4, "expected raw lx ~60620, got {}", js.l_x);

            // Reset to center, drain, then move into the 25% deadzone (lx = 0.2): RAW mode must report
            // both immediate and buffered raw physical (~39321) rather than deadzoned 0.
            backend_update(0, GcPad::default().to_xinput());
            assert_eq!((dvt.poll)(dev), DI_OK);
            n = 8;
            assert_eq!((dvt.get_device_data)(dev, 24, dod.as_mut_ptr() as *mut c_void, &mut n, 0), DI_OK);

            backend_update(0, GcPad { lx: 0.2, ..Default::default() }.to_xinput());
            assert_eq!((dvt.poll)(dev), DI_OK);
            n = 8;
            assert_eq!((dvt.get_device_data)(dev, 24, dod.as_mut_ptr() as *mut c_void, &mut n, 0), DI_OK);
            assert_eq!(n, 1);
            assert_eq!(dod[0].dw_ofs, 0);
            let buffered_raw = dod[0].dw_data as i32;
            assert!((buffered_raw - 39_321).abs() <= 4, "expected buffered raw lx ~39321, got {buffered_raw}");

            assert_eq!((dvt.get_device_state)(dev, 80, &mut js as *mut _ as *mut c_void), DI_OK);
            assert_eq!(js.l_x, buffered_raw);

            // Custom DIPROP_CALIBRATION shifts center to 39321: ignored in RAW, applied when back in COOKED.
            let cal = DIPROPCAL { diph: DIPROPHEADER { dw_size: 28, dw_header_size: 16, dw_obj: 0, dw_how: DIPH_BYOFFSET }, l_min: 0, l_center: 39_321, l_max: 65_535 };
            assert_eq!((dvt.set_property)(dev, DIPROP_CALIBRATION as *const Guid, &cal.diph), DI_OK);
            assert_eq!((dvt.get_device_state)(dev, 80, &mut js as *mut _ as *mut c_void), DI_OK);
            assert_eq!(js.l_x, buffered_raw);

            let cooked_mode = DIPROPDWORD { diph: DIPROPHEADER { dw_size: 20, dw_header_size: 16, dw_obj: 0, dw_how: DIPH_DEVICE }, dw_data: DIPROPCALIBRATIONMODE_COOKED };
            assert_eq!((dvt.set_property)(dev, DIPROP_CALIBRATIONMODE as *const Guid, &cooked_mode.diph), DI_OK);
            let dz0 = DIPROPDWORD { diph: DIPROPHEADER { dw_size: 20, dw_header_size: 16, dw_obj: 0, dw_how: DIPH_BYOFFSET }, dw_data: 0 };
            assert_eq!((dvt.set_property)(dev, DIPROP_DEADZONE as *const Guid, &dz0.diph), DI_OK);
            assert_eq!((dvt.get_device_state)(dev, 80, &mut js as *mut _ as *mut c_void), DI_OK);
            assert_eq!(js.l_x, 0);
            // Physical center (32767) is below the calibrated center (39321), so cooked output is negative.
            backend_update(0, GcPad::default().to_xinput());
            assert_eq!((dvt.get_device_state)(dev, 80, &mut js as *mut _ as *mut c_void), DI_OK);
            assert!(js.l_x < -150, "expected negative calibrated lx, got {}", js.l_x);

            backend_disconnect(0);
            assert_eq!((dvt.release)(dev), 0);
            assert_eq!((vt.release)(di), 0);
        }
    }
}
